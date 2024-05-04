mod app;
mod components;
mod events;
mod invoke;
mod layouts;
mod pages;
mod routes;

use app::App;

fn main() {
    sycamore::render(App);
}
