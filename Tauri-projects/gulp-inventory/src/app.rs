use std::time::Duration;

use leptos::leptos_dom::ev::SubmitEvent;
use leptos::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[component]
pub fn App() -> impl IntoView {
    let (greet_msg, set_greet_msg) = create_signal(String::from("Click me"));

    let greet = move |_| {
        spawn_local(async move {
            let args = to_value(&GreetArgs { name: "World" }).unwrap();
            // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
            let new_msg = invoke("greet", args).await.as_string().unwrap();
            set_greet_msg.update(|o| *o = new_msg);

            set_timeout(
                move || {
                    set_greet_msg.update(|g| *g = "Click again".to_string());
                },
                Duration::from_millis(1000),
            );
        });
    };

    view! {
        <div class="h-screen w-full flex flex-col items-center p-8">
            <button
                on:click=greet
                type="button"
                class="inline-block rounded bg-neutral-400 px-6 pb-2 pt-2.5 text-xs font-medium uppercase leading-normal text-white shadow-[0_4px_9px_-4px_#54b4d3] transition duration-150 ease-in-out hover:bg-info-600 hover:shadow-[0_8px_9px_-4px_rgba(84,180,211,0.3),0_4px_18px_0_rgba(84,180,211,0.2)] focus:bg-info-600 focus:shadow-[0_8px_9px_-4px_rgba(84,180,211,0.3),0_4px_18px_0_rgba(84,180,211,0.2)] focus:outline-none focus:ring-0 active:bg-info-700 active:shadow-[0_8px_9px_-4px_rgba(84,180,211,0.3),0_4px_18px_0_rgba(84,180,211,0.2)]">{ move || greet_msg.get() }</button>
        </div>
    }
}
