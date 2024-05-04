use sycamore::prelude::*;

#[component]
pub fn AnimalsPage<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        p {
            "Animals page"
        }
    }
}
