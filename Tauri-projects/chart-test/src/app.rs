use charts_rs::{PieChart, ScatterChart};
use leptos::leptos_dom::{ev::SubmitEvent, *};
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
    // let (name, set_name) = create_signal(String::new());
    // let (greet_msg, set_greet_msg) = create_signal(String::new());
    //
    // let update_name = move |ev| {
    //     let v = event_target_value(&ev);
    //     set_name.set(v);
    // };
    //
    // let greet = move |ev: SubmitEvent| {
    //     ev.prevent_default();
    //     spawn_local(async move {
    //         let name = name.get_untracked();
    //         if name.is_empty() {
    //             return;
    //         }
    //
    //         let args = to_value(&GreetArgs { name: &name }).unwrap();
    //         // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    //         let new_msg = invoke("greet", args).await.as_string().unwrap();
    //         set_greet_msg.set(new_msg);
    //     });
    // };
    //
    // view! {
    //     <main class="container">
    //         <div class="row">
    //             <a href="https://tauri.app" target="_blank">
    //                 <img src="public/tauri.svg" class="logo tauri" alt="Tauri logo"/>
    //             </a>
    //             <a href="https://docs.rs/leptos/" target="_blank">
    //                 <img src="public/leptos.svg" class="logo leptos" alt="Leptos logo"/>
    //             </a>
    //         </div>
    //
    //         <p>"Click on the Tauri and Leptos logos to learn more."</p>
    //
    //         <p>
    //             "Recommended IDE setup: "
    //             <a href="https://code.visualstudio.com/" target="_blank">"VS Code"</a>
    //             " + "
    //             <a href="https://github.com/tauri-apps/tauri-vscode" target="_blank">"Tauri"</a>
    //             " + "
    //             <a href="https://github.com/rust-lang/rust-analyzer" target="_blank">"rust-analyzer"</a>
    //         </p>
    //
    //         <form class="row" on:submit=greet>
    //             <input
    //                 id="greet-input"
    //                 placeholder="Enter a name..."
    //                 on:input=update_name
    //             />
    //             <button type="submit">"Greet"</button>
    //         </form>
    //
    //         <p><b>{ move || greet_msg.get() }</b></p>
    //     </main>
    // }
    let mut pie_chart = PieChart::new(vec![
        ("rose 1", vec![40.0]).into(),
        ("rose 2", vec![38.0]).into(),
        ("rose 3", vec![32.0]).into(),
        ("rose 4", vec![30.0]).into(),
        ("rose 5", vec![28.0]).into(),
        ("rose 6", vec![26.0]).into(),
        ("rose 7", vec![22.0]).into(),
        ("rose 8", vec![18.0]).into(),
        ("rose 9", vec![14.0]).into(),
        ("rose 10", vec![10.0]).into(),
        ("rose 11", vec![5.0]).into(),
    ]);
    pie_chart.title_text = "Nightingale Chart".to_string();
    pie_chart.sub_title_text = "Fake Data".to_string();

    let parser = web_sys::DomParser::new().unwrap();
    let parse_element = parser
        .parse_from_string(
            &pie_chart.svg().unwrap(),
            web_sys::SupportedType::ImageSvgXml,
        )
        .unwrap();

    let window = web_sys::window().expect("no global 'window' exists");
    let document = window.document().expect("should have a document on window");

    spawn_local(async move {
        let div_chart = document
            .get_element_by_id("pie-chart")
            .expect("no div name pie-chart");
        let _ = div_chart.append_child(&parse_element.first_child().expect("empty first_child"));
    });

    view! {
        <div id="pie-chart">
            "testing"
        </div>
    }
}
