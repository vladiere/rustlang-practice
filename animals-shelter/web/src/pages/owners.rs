use sycamore::prelude::*;

#[component]
pub fn OwnersPage<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        p {
            "Owners page"
        }
    }
}
