use leptos::prelude::*;

use crate::components::forms::FileUploadForm;

#[component]
pub fn Admin() -> impl IntoView {
    view! {
        <h1>"Welcome to admin panel"</h1>
        <FileUploadForm/>
    }
}
