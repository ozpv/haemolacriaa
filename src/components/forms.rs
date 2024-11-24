use crate::util::*;
use leptos::{ev::SubmitEvent, html::Input, *};
use server_fn::codec::{MultipartData, MultipartFormData};
use wasm_bindgen::JsCast;
use web_sys::{FormData, HtmlFormElement};

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use http::header::CONTENT_TYPE;
        use tokio::{fs::File, io::AsyncWriteExt};
        use crate::jwt::verify_jwt;
    }
}

/// TODO: upload in chunks with progress
/// and add multiple CONTENT_TYPE selections
#[server(input = MultipartFormData)]
pub async fn upload_file(data: MultipartData) -> Result<()> {
    let mut data = data.into_inner().unwrap();

    while let Ok(Some(field)) = data.next_field().await {
        let name = field
            .file_name()
            .expect("There to be a file name")
            .to_string();

        if field.headers().get(CONTENT_TYPE).unwrap() != "image/webp" {
            return err!("File is not WebP");
        }

        let filename = format!("/home/ozpv/Pictures/{name}");

        let mut file_handle = File::create(filename.clone()).await?;

        let bytes = field.bytes().await?;

        println!("Writing file {name} to {filename}");

        file_handle.write_all(&bytes).await?;
    }

    Ok(())
}

// TODO: call upload file from Action to display if there is an error
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

/// "login" with a valid token encoded with JWT_SECRET
#[server(Login, "/api", "Url")]
pub async fn login(token: String, redirect_url: Option<String>) -> Result<(), ServerFnError> {
    if verify_jwt(token).await.is_ok() {
        if let Some(redirect_url) = redirect_url {
            leptos_axum::redirect(&redirect_url);
        }
        Ok(())
    } else {
        err!("Failed to login")
    }
}

#[component]
pub fn LoginForm() -> impl IntoView {
    let token_input_element = create_node_ref::<Input>();

    // catch the login status
    // Option<Result<(), ServerFnError>>
    let login_action = create_action(|token: &String| {
        let token = token.clone();
        async move { login(token, Some("/admin".to_string())).await }
    });

    let on_submit = move |ev: SubmitEvent| {
        // prevent refresh
        ev.prevent_default();

        let token = token_input_element
            .get()
            .expect("token <input> tag should exist")
            .value();

        // call the login api
        login_action.dispatch(token);
    };

    view! {
        <form on:submit=on_submit>
            <label>"Enter a valid token:"</label>
            <input type="password" id="token-input" node_ref=token_input_element name="token-input"/>
            <button type="submit">"Login"</button>
        </form>

        // Info if token is valid
        {move || login_action.value().get().map_or(
            ().into_view(),
            // there is Some result
            |res| match res {
                Ok(_) => view! {
                    <p>"Logged in!"</p>
                },
                Err(_) => view! { <p>"Login failure. Please try again."</p> },
            }.into_view())
        }
    }
}
