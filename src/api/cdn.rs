use axum::{
    extract::{Path as AxumPath, Query, State},
    http::{header, HeaderValue, StatusCode},
    response::IntoResponse,
};
use axum_thiserror_tracing::IntoResponse;
use fast_image_resize::{images::Image, IntoImageView, Resizer};
use image::{codecs::webp::WebPEncoder, ImageReader};
use leptos::prelude::LeptosOptions;
use serde::Deserialize;
use std::{collections::HashSet, io::BufWriter, path::Path as FilePath, sync::LazyLock};
use thiserror::Error;
use tokio::{
    fs::write,
    fs::File,
    io::{AsyncReadExt, BufReader},
};

use crate::util::InsertMany;

#[derive(Deserialize)]
pub struct Dimensions {
    width: u32,
    height: u32,
}

impl Dimensions {
    fn supported(&self) -> bool {
        static SUPPORTED_DIMENSIONS: LazyLock<HashSet<(u32, u32)>> = LazyLock::new(|| {
            HashSet::from([
                (1000, 1000),
                (900, 900),
                (800, 800),
                (700, 700),
                (600, 600),
                (500, 500),
                (400, 400),
            ])
        });

        SUPPORTED_DIMENSIONS.contains(&(self.width, self.height))
    }
}

#[derive(Error, Debug, IntoResponse)]
pub enum CdnError {
    #[status(StatusCode::BAD_REQUEST)]
    #[error("Unsupported dimensions")]
    BadDimensions,
    #[status(StatusCode::BAD_REQUEST)]
    #[error("The requested resource must be a WebP image")]
    IncorrectFormat,
    #[status(StatusCode::NOT_FOUND)]
    #[error("The requested resource was not found on the server")]
    NotFound,
    #[error("{0}")]
    Internal(&'static str),
}

/// serves webp images and resizes them
/// GET `/assets/{file_name}.webp?width={width}&height={height}`
///
/// # Errors
///
/// if the image isn't .webp or if the dimensions are missing or unsupported
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

    let mut plain_path = FilePath::new(&site_root.to_string()).join(file_name);
    plain_path.set_extension("webp");

    let img_path = FilePath::new(&site_root.to_string()).join(format!(
        "{}-{}x{}.webp",
        file_name, dimensions.width, dimensions.height
    ));

    // if the image is already resized then just send it
    if img_path.exists() {
        let mut image = Vec::with_capacity(200_000);

        let file = File::open(img_path)
            .await
            .map_err(|_| CdnError::Internal("Something went wrong"))?;

        let mut buf_reader = BufReader::new(file);

        let mut length = 0;
        let mut res = buf_reader
            .read_to_end(&mut image)
            .await
            .map(|len| {
                length = len;
                (StatusCode::OK, image).into_response()
            })
            .map_err(|_| CdnError::Internal("Failed to read image"))?;

        #[rustfmt::skip]
        res.headers_mut().insert_many([
            (header::CONTENT_LENGTH, length.into()),
            (header::CONTENT_TYPE, HeaderValue::from_static("image/webp")),
            (header::ACCEPT_RANGES, HeaderValue::from_static("bytes")),
            (header::CACHE_CONTROL, HeaderValue::from_static("public, max-age=15552000")),
        ]);

        Ok(res)
        // if the image doesn't exist, resize it
    } else if plain_path.exists() {
        let image = tokio::task::spawn_blocking(move || {
            let original_image = ImageReader::open(plain_path)
                .ok()
                .and_then(|image| image.decode().ok())
                .ok_or(CdnError::Internal("Something went wrong reading an image"))?;

            let mut resized_image = Image::new(
                dimensions.width,
                dimensions.height,
                original_image
                    .pixel_type()
                    .ok_or(CdnError::Internal("Failed to detect pixel type"))?,
            );

            let mut resizer = Resizer::new();
            resizer
                .resize(&original_image, &mut resized_image, None)
                .map_err(|_| CdnError::Internal("Failed to resize image"))?;

            let mut result_image = BufWriter::new(Vec::new());
            WebPEncoder::new_lossless(&mut result_image)
                .encode(
                    resized_image.buffer(),
                    dimensions.width,
                    dimensions.height,
                    original_image.color().into(),
                )
                .map_err(|_| CdnError::Internal("Failed to encode image"))?;

            let image = result_image
                .into_inner()
                .map_err(|_| CdnError::Internal("Failed to flush buffer"))?;

            Ok(image)
        })
        .await
        .map_err(|_| CdnError::Internal("Blocking task panicked"))??;

        // save on the server
        _ = write(img_path, &image).await;

        let length = image.len();
        let mut res = (StatusCode::OK, image).into_response();

        #[rustfmt::skip]
        res.headers_mut().insert_many([
            (header::CONTENT_LENGTH, length.into()),
            (header::CONTENT_TYPE, HeaderValue::from_static("image/webp")),
            (header::ACCEPT_RANGES, HeaderValue::from_static("bytes")),
            (header::CACHE_CONTROL, HeaderValue::from_static("public, max-age=15552000")),
        ]);

        Ok(res)
    } else {
        Err(CdnError::NotFound)
    }
}
