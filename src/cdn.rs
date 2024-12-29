use axum::{
    async_trait,
    extract::{
        path::ErrorKind, rejection::PathRejection, FromRequestParts, Path as AxumPath, Query, State,
    },
    http::{request::Parts, StatusCode},
    response::IntoResponse,
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

// TODO: make a generic to handle all types of images
pub async fn handle_webp_image(
    ImageFile(file_name): ImageFile<String>,
    dimensions: Query<Dimensions>,
    State(leptos_options): State<LeptosOptions>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    if dimensions.width % 10 != 0 || dimensions.height % 10 != 0 {
        return Err((
            StatusCode::BAD_REQUEST,
            "Dimensions must be multiples of 10",
        ));
    }

    if !file_name.contains(".webp") {
        return Err((StatusCode::BAD_REQUEST, "Requested file must be a .webp"));
    }

    let site_root = leptos_options.site_root;

    let plain_path = FilePath::new(&site_root.to_string()).join(&file_name);

    let img_path = FilePath::new(&site_root.to_string()).join(format!(
        "{}-{}x{}.webp",
        file_name.strip_suffix(".webp").unwrap_or(""),
        dimensions.width,
        dimensions.height
    ));

    tracing::info!("Checking if {} exists", img_path.display());

    if img_path.exists() {
        let file = File::open(img_path).map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to open requested file on server",
            )
        })?;
        let mut buf_reader = BufReader::new(file);

        let mut image = Vec::with_capacity(100000);

        buf_reader.read_to_end(&mut image).map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Located the requested file on the server but failed to read it",
            )
        })?;

        tracing::info!(
            "Got file: {file_name} ({}px, {}px)",
            dimensions.width,
            dimensions.height
        );

        Ok((StatusCode::OK, image))
    } else if plain_path.exists() {
        tracing::info!(
            "Requested file {} doesn't exist resizing it",
            plain_path.display()
        );

        let dyn_image = ImageReader::open(plain_path)
            .map_err(|_| (StatusCode::BAD_REQUEST, "File not found on the server"))?
            .decode()
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to decode the request file",
                )
            })?
            .resize_exact(dimensions.width, dimensions.height, FilterType::Lanczos3);

        let mut image = Vec::with_capacity(100000);

        dyn_image
            .write_to(&mut Cursor::new(&mut image), ImageFormat::WebP)
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to write DynamicImage to the image buffer",
                )
            })?;

        // write the image on the server for caching
        let _ = write(img_path, image.clone());

        Ok((StatusCode::OK, image))
    } else {
        Err((
            StatusCode::BAD_REQUEST,
            "Requested file not found on the server",
        ))
    }
}
