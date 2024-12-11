use crate::util::err;
use leptos::{ev::SubmitEvent, html::Input, prelude::*, task::spawn_local};
use server_fn::codec::{MultipartData, MultipartFormData};
use wasm_bindgen::JsCast;
use web_sys::{FormData, HtmlFormElement};

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use http::{StatusCode, header::CONTENT_TYPE};
        use tokio::{fs::File, io::AsyncWriteExt};
        use crate::jwt::verify_jwt;
        use leptos_axum::{extract, ResponseOptions};
        use http::{HeaderMap, HeaderValue, header};
        use time::Duration;
        use cookie::{SameSite, Cookie};
    }
}

/// TODO: upload in chunks with progress
/// and add multiple CONTENT_TYPE selections
#[server(input = MultipartFormData, prefix = "/api/opr")]
pub async fn upload_file(data: MultipartData) -> Result<(), ServerFnError> {
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
#[server(Login, "/api", "Url", "login")]
pub async fn login(token: String, redirect_url: Option<String>) -> Result<(), ServerFnError> {
    if verify_jwt(token.clone()).await.is_ok() {
        let response = expect_context::<ResponseOptions>();

        let cookie = Cookie::build(("tok", &token))
            .secure(true)
            .http_only(true)
            .same_site(SameSite::None)
            .max_age(Duration::days(7))
            .build();

        let Ok(header_value) = HeaderValue::from_str(&cookie.to_string()) else {
            expect_context::<ResponseOptions>().set_status(StatusCode::INTERNAL_SERVER_ERROR);
            return err!("Something went wrong");
        };

        response.insert_header(header::SET_COOKIE, header_value);

        if let Some(redirect_url) = redirect_url {
            leptos_axum::redirect(&redirect_url);
        }
        Ok(())
    } else {
        expect_context::<ResponseOptions>().set_status(StatusCode::UNAUTHORIZED);
        err!("Failed to login")
    }
}

/// Similar to the middleware
#[server(LoggedIn, "/api", "Url", "logged_in")]
pub async fn logged_in() -> Result<(), ServerFnError> {
    let headers = extract::<HeaderMap>().await?;

    println!("checking login cookie");
    // TODO: make this pattern simpler
    let Some(token) = headers.get_all(header::COOKIE).iter().find_map(|cookie| {
        println!("parsing cookie");
        let cookie =
            Cookie::parse(cookie.to_str().expect("Cookie to parse")).expect("Cookie is broken");

        if cookie.name() == "tok" {
            Some(cookie.value().to_string())
        } else {
            None
        }
    }) else {
        println!("Failed to find cookie");
        return err!("Failed to find cookie in header");
    };

    if verify_jwt(token).await.is_ok() {
        Ok(())
    } else {
        err!("Failed to login, token is invalid")
    }
}

#[component]
pub fn LoginForm() -> impl IntoView {
    let token_input_element = NodeRef::<Input>::new();

    // catch the login status
    let login_action = Action::new(|token: &String| {
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
        {move || login_action.value().get().map_or(().into_any(),
            // there is Some result
            |res| match res {
                Ok(()) => view! {
                    <p>"Logged in!"</p>
                }.into_any(),
                Err(_) => view! { <p class="text-red-dark">"Login failure. Please try again."</p> }.into_any(),
            })
        }

        <a href="/admin">"Goto admin"</a>
    }
}
