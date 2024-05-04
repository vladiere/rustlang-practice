use sycamore::prelude::*;

#[component(inline_props)]
pub fn AnchorTag<G: Html>(cx: Scope, route_path: String, content: String) -> View<G> {
    view! { cx,
        a(href=route_path, class="hover:text-blue-600 pb-1 px-2 border-b border-gray-600 hover:border-blue-600") { (content) }
    }
}
