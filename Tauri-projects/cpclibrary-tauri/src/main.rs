pub mod app;
pub mod components;
pub mod layouts;
pub mod pages;
pub mod services;
pub mod types;

use app::*;
use leptos::*;

fn main() {
    mount_to_body(|| {
        view! {
            <App/>
        }
    })
}
