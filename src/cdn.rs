use axum::{
    async_trait,
    extract::{
        path::ErrorKind, rejection::PathRejection, FromRequestParts, Path as AxumPath, Query, State,
    },
    http::{header, request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use image::{imageops::FilterType, ImageFormat, ImageReader};
use leptos::prelude::LeptosOptions;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{
    fs::{write, File},
    io::{BufReader, Cursor, Read},
    path::Path as FilePath,
};
use thiserror::Error;
use tokio::task;

#[derive(Serialize)]
pub struct ImageFileError {
    message: String,
    location: Option<String>,
}

pub struct ImageFile<T>(T);

#[async_trait]
impl<T, S> FromRequestParts<S> for ImageFile<T>
where
    T: DeserializeOwned + Send,
    S: Send + Sync,
{
    type Rejection = (StatusCode, Json<ImageFileError>);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let path_frq = match AxumPath::<T>::from_request_parts(parts, state).await {
            Ok(value) => Ok(Self(value.0)),
            Err(rejection) => {
                let (status, body) = match rejection {
                    PathRejection::FailedToDeserializePathParams(inner) => {
                        let mut status = StatusCode::BAD_REQUEST;

                        let kind = inner.into_kind();
                        let body = match &kind {
                            ErrorKind::WrongNumberOfParameters { .. } => ImageFileError {
                                message: kind.to_string(),
                                location: None,
                            },

                            ErrorKind::ParseErrorAtKey { key, .. } => ImageFileError {
                                message: kind.to_string(),
                                location: Some(key.clone()),
                            },

                            ErrorKind::ParseErrorAtIndex { index, .. } => ImageFileError {
                                message: kind.to_string(),
                                location: Some(index.to_string()),
                            },

                            ErrorKind::ParseError { .. } => ImageFileError {
                                message: kind.to_string(),
                                location: None,
                            },

                            ErrorKind::InvalidUtf8InPathParam { key } => ImageFileError {
                                message: kind.to_string(),
                                location: Some(key.clone()),
                            },

                            ErrorKind::UnsupportedType { .. } => {
                                status = StatusCode::INTERNAL_SERVER_ERROR;
                                ImageFileError {
                                    message: kind.to_string(),
                                    location: None,
                                }
                            }

                            ErrorKind::Message(msg) => ImageFileError {
                                message: msg.clone(),
                                location: None,
                            },

                            _ => ImageFileError {
                                message: format!("Unhandled deserialization error: {kind}"),
                                location: None,
                            },
                        };

                        (status, body)
                    }
                    PathRejection::MissingPathParams(error) => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        ImageFileError {
                            message: error.to_string(),
                            location: None,
                        },
                    ),
                    _ => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        ImageFileError {
                            message: format!("Unhandled path rejection: {rejection}"),
                            location: None,
                        },
                    ),
                };

                Err((status, axum::Json(body)))
            }
        };
        // add extra checks
        path_frq
    }
}

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
    #[error("Failed to execute spawn_blocking")]
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

        (status, format!("{}", self)).into_response()
    }
}

// TODO: make a generic to handle all types of images
pub async fn handle_webp_image(
    ImageFile(file_name): ImageFile<String>,
    dimensions: Query<Dimensions>,
    State(leptos_options): State<LeptosOptions>,
) -> Result<impl IntoResponse, CdnError> {
    if !dimensions.supported() {
        return Err(CdnError::BadDimensions);
    }

    if file_name.strip_suffix(".webp").is_none() {
        return Err(CdnError::IncorrectFormat);
    }

    let site_root = leptos_options.site_root;

    let plain_path = FilePath::new(&site_root.to_string()).join(&file_name);

    let img_path = FilePath::new(&site_root.to_string()).join(format!(
        "{}-{}x{}.webp",
        file_name.strip_suffix(".webp").unwrap_or(""),
        dimensions.width,
        dimensions.height
    ));

    #[allow(unused_mut)]
    let (mut res, len) = if img_path.exists() {
        let res = task::spawn_blocking(move || {
            let file = File::open(img_path).map_err(|_| CdnError::FileOpenError)?;
            let mut buf_reader = BufReader::new(file);

            // ~200000 bytes is around the size of a 400x400 webp
            let mut image = Vec::with_capacity(200000);

            buf_reader
                .read_to_end(&mut image)
                .map_err(|_| CdnError::ReadError)?;

            let len = image.len();
            let mut res = (StatusCode::OK, image).into_response();

            Ok((res, len))
        })
        .await
        .map_err(|_| CdnError::SpawnBlockingError)?;

        res?
    } else if plain_path.exists() {
        tracing::info!(
            "Requested file {} doesn't exist; resizing it",
            plain_path.display()
        );

        let res = task::spawn_blocking(move || {
            let dyn_image = ImageReader::open(plain_path)
                .map_err(|_| CdnError::FileOpenError)?
                .decode()
                .map_err(|_| CdnError::DecodeError)?
                .resize_exact(dimensions.width, dimensions.height, FilterType::Lanczos3);

            let mut image = Vec::with_capacity(200000);

            dyn_image
                .write_to(&mut Cursor::new(&mut image), ImageFormat::WebP)
                .map_err(|_| CdnError::BufWriteError)?;

            // write the image on the server for caching
            let _ = write(img_path, image.clone());

            let len = image.len();
            let mut res = (StatusCode::OK, image).into_response();

            Ok((res, len))
        })
        .await
        .map_err(|_| CdnError::SpawnBlockingError)?;

        res?
    } else {
        return Err(CdnError::NotFound);
    };

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
