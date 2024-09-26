use crate::lazy::IMAGE_UPLOAD_DIR;
use axum::{body::Body, extract::Multipart, response::IntoResponse, response::Response, Json};
use http::StatusCode;
use serde_json::json;
use tokio::fs::{metadata, File};
use tokio::io::AsyncWriteExt;

#[server(input = MultipartFormData)]
pub async fn upload_file(multipart: MultipartData) -> Result<(), ServerFnError> {
    let mut multipart = multipart.into_inner().unwrap();

    while let Ok(Some(field)) = multipart.next_field().await {
        let name = field.name().unwrap().to_string();

        if name == "file" {
            let content_type = field.content_type().unwrap();

            if content_type != "image/webp" {
                return Err(ServerFnError::new("Incorrect File Type!");
            }

            let file_name = field.file_name().unwrap();
            let file_name = format!("{}/{file_name}", IMAGE_UPLOAD_DIR.as_str());

            if metadata(&file_name).await.is_ok() {
                return Err(ServerFnError::new("File already exists on server!");
            }

            let mut handle = File::create(file_name)
                .await
                .expect("Failed to open file creation handle!");

            let bytes = field.bytes().await.unwrap();

            handle
                .write_all(&bytes)
                .await
                .expect("Failed to write image data!");
        }
    }
    
    Ok(())
}
