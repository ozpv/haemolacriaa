#[cfg(feature = "ssr")]
use http::header::CONTENT_TYPE;
use leptos::ev::SubmitEvent;
use leptos::*;
use server_fn::codec::{MultipartData, MultipartFormData};
#[cfg(feature = "ssr")]
use tokio::{fs::File, io::AsyncWriteExt};
use wasm_bindgen::JsCast;
use web_sys::{FormData, HtmlFormElement};

/// TODO: upload in chunks with progress
/// and add multiple CONTENT_TYPE selections

#[server(input = MultipartFormData)]
pub async fn upload_file(data: MultipartData) -> Result<(), ServerFnError> {
    let mut data = data.into_inner().unwrap();

    while let Ok(Some(field)) = data.next_field().await {
        let name = field.file_name().expect("No filename on field").to_string();

        if field.headers().get(CONTENT_TYPE).unwrap() != "image/webp" {
            return Err(ServerFnError::new("File is not WebP"));
        }

        let filename = format!("/home/ozpv/Pictures/{name}");

        let mut file_handle = File::create(filename.clone()).await?;

        let bytes = field.bytes().await?;

        println!("Writing file {name} to {filename}");

        file_handle.write_all(&bytes).await?;
    }

    Ok(())
}

#[component]
pub fn FileUploadForm() -> impl IntoView {
    // upload file on submit
    let on_submit = move |ev: SubmitEvent| {
        // prevent refresh
        ev.prevent_default();

        let target = ev.target().unwrap().unchecked_into::<HtmlFormElement>();
        let data = FormData::new_with_form(&target).unwrap();

        // upload the file
        spawn_local(async move {
            upload_file(data.into())
                .await
                .expect("Failed to upload file");
        });
    };

    view! {
        <form on:submit=on_submit>
            <label for="file-input">"Choose a WebP"</label>
            <input type="file" id="file-input" name="file" accept="image/webp"/>
            <button type="submit">"Upload"</button>
        </form>
    }
}
