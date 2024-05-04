use sycamore::prelude::*;

#[component(inline_props)]
pub fn TitleTag<G: Html>(cx: Scope, title: String) -> View<G> {
    view! { cx,
        h1(class="text-8md font-bold text-white") {
            (title)
        }
    }
}
