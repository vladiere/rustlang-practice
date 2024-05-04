mod dashboard;

pub use dashboard::*;

use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn MainView() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    view! {
        <Box class="flex flex-col items-center h-screen min-w-full">
            <H2>"Welcome to Leptonic"</H2>

            <span style="margin-top: 3em;">"Count: " {move || count.get()}</span>
            <Button class="bg-green-800" on_click=move|_| set_count.update(|c| *c += 1)>
                "Increase"
            </Button>
        </Box>
    }
}
