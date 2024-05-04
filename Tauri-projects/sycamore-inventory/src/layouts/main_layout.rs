use crate::components::{anchor::AnchorTag, title::TitleTag};
use sycamore::prelude::*;

#[component]
pub fn MainLayout<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        div(class="w-[15%]") {
            div(class="h-screen pt-5 flex flex-col gap-10 items-center border-r-2 border-gray-400") {
                TitleTag(title="Gulp Inventory".to_string())
                AnchorTag(route_path="/".to_string(), content="Home".to_string())
                AnchorTag(route_path="/owner".to_string(), content="Owners".to_string())
                AnchorTag(route_path="/animal".to_string(), content="Animals".to_string())
                AnchorTag(route_path="/add".to_string(), content="Add new".to_string())
            }
        }
    }
}
