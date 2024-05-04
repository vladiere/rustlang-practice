use sycamore::prelude::*;

#[component]
pub fn Image<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        img(src="public/sycamore.svg",class="logo sycamore",alt="Sycamore logo")
    }
}
