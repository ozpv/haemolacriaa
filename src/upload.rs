use crate::lazy::IMAGE_UPLOAD_DIR;
use axum::{body::Body, extract::Multipart, response::IntoResponse, response::Response, Json};
use http::StatusCode;
use serde_json::json;
use tokio::fs::{metadata, File};
use tokio::io::AsyncWriteExt;

pub enum UploadError {
    AlreadyExists,
    IncorrectFileType,
}

impl IntoResponse for UploadError {
    fn into_response(self) -> Response {
        use UploadError::*;
        let (status, message) = match self {
            AlreadyExists => (StatusCode::BAD_REQUEST, "File already exists on the server"),
            IncorrectFileType => (StatusCode::BAD_REQUEST, "Provided file is not WebP!"),
        };

        let body = Json(json!({
            "error": message,
        }));

        (status, body).into_response()
    }
}

pub async fn upload_image(mut multipart: Multipart) -> Result<Response<Body>, UploadError> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();

        if name == "file" {
            let file_name = field.file_name().unwrap();

            if file_name.chars().rev().take(5).collect::<String>() != "pbew." {
                return Err(UploadError::IncorrectFileType);
            }

            let file_name = format!("{}/{file_name}", IMAGE_UPLOAD_DIR.as_str());

            let data = field.bytes().await.unwrap();

            if metadata(&file_name).await.is_ok() {
                return Err(UploadError::AlreadyExists);
            }

            let mut handle = File::create(file_name)
                .await
                .expect("Failed to open file creation handle!");

            handle
                .write_all(&data)
                .await
                .expect("Failed to write image data!");
        }
    }
    Ok(StatusCode::OK.into_response())
}
