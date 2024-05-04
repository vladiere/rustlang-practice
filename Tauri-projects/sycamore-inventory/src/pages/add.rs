use sycamore::prelude::*;

use crate::components::{add_animal::AddAnimal, add_owner::AddOwner, title::TitleTag};

#[component]
pub fn AddNewPage<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        div(class="w-full h-full p-2 flex flex-row justify-center") {
            div(class="w-full flex flex-col items-center") {
                TitleTag(title="Add new animal".to_string())
                AddAnimal {}
            }
            div(class="w-full flex flex-col items-center") {
                TitleTag(title="Add new owner".to_string())
                AddOwner {}
            }
        }
    }
}
