use leptonic::prelude::*;
use leptos::*;
use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum AppError {
    #[error("Not Found")]
    NotFound,
}

// A basic function to display errors served by the error boundaries.
// Feel free to do more complicated things here than just displaying the error.
#[component]
pub fn ErrorTemplate(
    #[prop(optional)] outside_errors: Option<Errors>,
    #[prop(optional)] errors: Option<RwSignal<Errors>>,
) -> impl IntoView {
    let errors = match outside_errors {
        Some(e) => create_rw_signal(e),
        None => match errors {
            Some(e) => e,
            None => panic!("No Errors found and we expected errors!"),
        },
    };
    // Get Errors from Signal
    let errors = errors.get_untracked();

    // Downcast lets us take a type that implements `std::error::Error`
    let errors: Vec<AppError> = errors
        .into_iter()
        .filter_map(|(_k, v)| v.downcast_ref::<AppError>().cloned())
        .collect();
    println!("Errors: {errors:#?}");

    let num_errors = errors.len();

    view! {
        <Box class="h-screen min-w-full flex flex-col items-center justify-center gap-10">
            <H1 class="text-6xl">{match num_errors {
                1 => "Error",
                _ => "Errors",
            }}</H1>

            <For
                each=move || { errors.clone().into_iter().enumerate() }
                key=|(index, _error)| *index
                children=move |(_index, error)| {
                    match error {
                        AppError::NotFound => view! {
                            <P class="text-2xl">"404 - Not Found"</P>
                        },
                    }
                }
            />

            <LinkButton href="/home">
                "Back"
            </LinkButton>
        </Box>
    }
}
