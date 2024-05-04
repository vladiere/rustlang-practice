use crate::invoke::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use sycamore::{futures::spawn_local_scoped, prelude::*, rt::Event};

#[derive(Serialize, Deserialize)]
struct OwnerArgs<'a> {
    name: &'a str,
}

#[component]
pub fn AddOwner<G: Html>(cx: Scope) -> View<G> {
    let owner_name = create_signal(cx, String::new());
    let errors = create_signal(cx, String::new());

    let create_owner = move |ev: Event| {
        ev.prevent_default();

        if owner_name.get().is_empty() {
            return;
        } else if owner_name.get().len() < 8 {
            errors.set("Must be at least 8 characters".to_string());
            log(&String::from("Must be at least 8 characters"));
            return;
        }

        spawn_local_scoped(cx, async move {
            let new_greet = invoke(
                "greet",
                to_value(&OwnerArgs {
                    name: &owner_name.get(),
                })
                .unwrap(),
            )
            .await;

            errors.set(String::new());
            log(&new_greet.as_string().unwrap());
        })
    };

    view! { cx,
        form(class="w-[50%] flex flex-col mt-5 gap-5", on:submit=create_owner) {
            div {
                label(for="owner_name", class="block mb-2 text-sm font-medium text-white") { "Owner name" }
                input(type="text", id="owner_name", bind:value=owner_name, class="outline-none border border-gray-300 text-gray-900 text-sm rounded-lg focus:border-blue-500 block w-full p-2.5 bg-gray-700 border-gray-600 placeholder-gray-400 text-white focus:ring-blue-500 dark:focus:border-blue-500", placeholder="e.g. John", required=true )
                span(class="text-red-500 font-medium") { (errors.get()) }
            }
            button(type="submit", class="text-white self-start outline-none bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800") {
                "Submit"
            }
        }
    }
}
