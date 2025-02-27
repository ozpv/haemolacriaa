use axum::{
    extract::{Path as AxumPath, Query, State},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
};
use image::{imageops::FilterType, ImageFormat, ImageReader};
use leptos::prelude::LeptosOptions;
use serde::Deserialize;
use std::{
    fs::{write, File},
    io::{BufReader, Cursor, Read},
    path::Path as FilePath,
};
use thiserror::Error;
use tokio::sync::oneshot;

#[derive(Deserialize)]
pub struct Dimensions {
    width: u32,
    height: u32,
}

impl Dimensions {
    fn supported(&self) -> bool {
        match (self.width, self.height) {
            (1000, 1000) => true,
            (900, 900) => true,
            (800, 800) => true,
            (700, 700) => true,
            (600, 600) => true,
            (500, 500) => true,
            (400, 400) => true,
            _ => false,
        }
    }
}

#[derive(Error, Debug)]
pub enum CdnError {
    #[error("Unsupported dimensions")]
    BadDimensions,
    #[error("The requested resource must be a WebP image")]
    IncorrectFormat,
    #[error("Failed to open the requested file on the server")]
    FileOpenError,
    #[error("Failed to decode the requested file")]
    DecodeError,
    #[error("Located the requested resource on the server but failed to read it")]
    ReadError,
    #[error("The requested resource was not found on the server")]
    NotFound,
    #[error("Failed to spawn blocking task")]
    SpawnBlockingError,
    #[error("Failed to write DynamicImage to the response buffer")]
    BufWriteError,
    #[error("Failed to set the CONTENT_TYPE header")]
    ResponseError,
}

impl IntoResponse for CdnError {
    fn into_response(self) -> Response {
        let status = match self {
            Self::BadDimensions => StatusCode::BAD_REQUEST,
            Self::IncorrectFormat => StatusCode::BAD_REQUEST,
            Self::NotFound => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status, self.to_string()).into_response()
    }
}

// TODO: make a generic to handle all types of images
pub async fn handle_webp_image(
    AxumPath(file_name): AxumPath<String>,
    dimensions: Query<Dimensions>,
    State(leptos_options): State<LeptosOptions>,
) -> Result<impl IntoResponse, CdnError> {
    if !dimensions.supported() {
        return Err(CdnError::BadDimensions);
    }

    let Some(file_name) = file_name.strip_suffix(".webp") else {
        return Err(CdnError::IncorrectFormat);
    };

    let site_root = leptos_options.site_root;

    let mut plain_path = FilePath::new(&site_root.to_string()).join(&file_name);
    plain_path.set_extension("webp");

    let img_path = FilePath::new(&site_root.to_string()).join(format!(
        "{}-{}x{}.webp",
        file_name, dimensions.width, dimensions.height
    ));

    // either get the file if it already was resized
    // or resize it
    let (tx, rx) = oneshot::channel();
    rayon::spawn(move || {
        let mut image = Vec::with_capacity(200000);

        let res = if img_path.exists() {
            File::open(img_path).map_or_else(
                |_| Err(CdnError::FileOpenError),
                |file| {
                    let mut buf_reader = BufReader::new(file);

                    buf_reader.read_to_end(&mut image).map_or_else(
                        |_| Err(CdnError::ReadError),
                        |len| Ok(((StatusCode::OK, image).into_response(), len)),
                    )
                },
            )
        } else if plain_path.exists() {
            ImageReader::open(plain_path).map_or_else(
                |_| Err(CdnError::FileOpenError),
                |reader| {
                    reader.decode().map_or_else(
                        |_| Err(CdnError::DecodeError),
                        |reader| {
                            reader
                                .resize_exact(
                                    dimensions.width,
                                    dimensions.height,
                                    FilterType::Lanczos3,
                                )
                                .write_to(&mut Cursor::new(&mut image), ImageFormat::WebP)
                                .map_or_else(
                                    |_| Err(CdnError::BufWriteError),
                                    |()| {
                                        // save the image on the server for caching
                                        let _ = write(img_path, image.clone());
                                        let len = image.len();
                                        Ok(((StatusCode::OK, image).into_response(), len))
                                    },
                                )
                        },
                    )
                },
            )
        } else {
            Err(CdnError::NotFound)
        };

        let _ = tx.send(res);
    });

    let (mut res, len) = rx.await.map_err(|_| CdnError::SpawnBlockingError)??;

    // set up response
    res.headers_mut().insert(
        header::CONTENT_TYPE,
        "image/webp".parse().map_err(|_| CdnError::ResponseError)?,
    );

    res.headers_mut().insert(header::CONTENT_LENGTH, len.into());

    res.headers_mut().insert(
        header::ACCEPT_RANGES,
        "bytes".parse().map_err(|_| CdnError::ResponseError)?,
    );

    // cache this image for up to 6 months
    res.headers_mut().insert(
        header::CACHE_CONTROL,
        "public, max-age=15552000"
            .parse()
            .map_err(|_| CdnError::ResponseError)?,
    );

    Ok(res)
}
