mod app;
mod testing;

use app::App;

fn main() {
    sycamore::render(App);
}
