use crate::components::button::Button;
use crate::components::count::Count;
use crate::components::dropdown::Dropdown;
use crate::components::square::Square;
use sycamore::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;

#[component]
fn HelloWorld<G: Html>(cx: Scope) -> View<G> {
    let state = create_signal(cx, 0i32);
    let update = create_ref(cx, |action| {
        if action == 0 {
            state.set(0);
        } else {
            state.set(*state.get() + action);
        }
    });

    let show_dropdown = create_signal(cx, false);

    view! {cx,
        article(class="flex flex-col justify-center items-center") {
            Count(value=state)
            header {
                Button(updater=update,action=-1)
                Button(updater=update,action=0)
                Button(updater=update,action=1)
            }
            Square(value=state)
        }

        button(id="dropdownDefaultButton", data-dropdown-toggle="dropdown", class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center inline-flex items-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800", type="button", on:click=move |_| {
            let value = if *show_dropdown.get() { false } else { true };

            show_dropdown.set(value);
        }) {
            "Dropdown button"
            svg(class="w-2.5 h-2.5 ms-3", aria-hidden="true", xmlns="http://www.w3.org/2000/svg", fill="none", viewBox="0 0 10 6") {
                path(stroke="currentColor", stroke-linecap="round", stroke-linejoin="round", stroke-width="2", d="m1 1 4 4 4-4")
            }
        }

        Dropdown(value=show_dropdown)
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    sycamore::render(HelloWorld)
}
