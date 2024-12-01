use http::StatusCode;
use leptos::prelude::*;
use thiserror::Error;

use crate::components::{footer::Footer, nav::Nav};

#[derive(Clone, Debug, Error)]
pub enum AppError {
    #[error("Not Found")]
    NotFound,
}

impl AppError {
    pub fn status_code(&self) -> StatusCode {
        use AppError::*;
        match self {
            NotFound => StatusCode::NOT_FOUND,
        }
    }
}

#[component]
pub fn ErrorPage(
    #[prop(optional)] outside_errors: Option<Errors>,
    #[prop(optional)] errors: Option<RwSignal<Errors>>,
) -> impl IntoView {
    let errors = match outside_errors {
        Some(e) => RwSignal::new(e),
        None => match errors {
            Some(e) => e,
            None => panic!("No Errors found and we expected errors!"),
        },
    };
    let errors = errors.get_untracked();

    let errors: Vec<AppError> = errors
        .into_iter()
        .filter_map(|(_k, v)| v.downcast_ref::<AppError>().cloned())
        .collect();
    println!("Error: {errors:?}");

    #[cfg(feature = "ssr")]
    {
        use leptos_axum::ResponseOptions;
        let response = use_context::<ResponseOptions>();
        if let Some(response) = response {
            response.set_status(errors[0].status_code());
        }
    }

    view! {
        <Nav/>
        <div class="bg-gray-900 min-h-screen">
            <For
                each=move || {errors.clone().into_iter().enumerate()}
                key=|(index, _error)| *index
                children=move |error| {
                    //let error_string = error.1.to_string();
                    let error_code = error.1.status_code();
                    view! {
                        <h2 class="text-white text-center text-2xl font-sans p-[20px]">{error_code.to_string()}</h2>
                        //<p class="text-white text-center text-lg font-sans">"Error: " {error_string}</p>
                        <a class="block text-white justify-center text-center font-sans hover:text-blue-900" href="/">{"Return home"}</a>
                    }
                }
            />
        </div>
        <Footer/>
    }
}
