use leptos::{ev::SubmitEvent, html, leptos_dom::logging::console_log, prelude::*};
use thiserror::Error;

#[cfg(feature = "ssr")]
use base64::{engine::general_purpose, Engine};

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Something went wrong")]
    Internal,
}

#[cfg(feature = "ssr")]
#[inline]
pub fn gen_rand_string<const N: usize>() -> String {
    let mut bytes = [0u8; N];

    rand::fill(&mut bytes[..]);

    general_purpose::STANDARD.encode(bytes)
}

#[server]
pub async fn get_csrf_token() -> Result<String, ServerFnError> {
    // TODO: add to a session store
    Ok(gen_rand_string::<32>())
}

#[component]
pub fn CsrfForm(
    on_submit: Box<dyn Fn(SubmitEvent, String) + Send + Sync>,
    children: Children,
) -> impl IntoView {
    let csrf_token = OnceResource::new_blocking(get_csrf_token());

    let csrf = NodeRef::<html::Input>::new();

    let onsubmit = move |ev: SubmitEvent| {
        let token = csrf.get().unwrap().value();
        on_submit(ev, token);
    };

    let token_element = Suspend::new(async move {
        match csrf_token.await {
            Ok(token) => view! {
                <input type="hidden" value=token node_ref=csrf />
            }
            .into_any(),
            Err(_) => view! {
                <a>"Something went wrong"</a>
            }
            .into_any(),
        }
    });

    view! {
        <Suspense fallback=|| view! {
            <a>"Loading..."</a>
        }>
            <form on:submit=onsubmit>
                {token_element}
                {children()}
            </form>
        </Suspense>
    }
}

#[component]
pub fn CsrfFormTest() -> impl IntoView {
    let text = NodeRef::<html::Input>::new();

    view! {
        <CsrfForm on_submit=Box::new(move |ev, token| {
            ev.prevent_default();

            let value = text.get().unwrap().value();
            console_log(&format!("{value} {token}"));
        })>
            <input type="text" node_ref=text />
            <button type="submit">"Submit"</button>
        </CsrfForm>
    }
}
