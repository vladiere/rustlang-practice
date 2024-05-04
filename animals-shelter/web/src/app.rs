use crate::pages::*;
use sycamore::prelude::*;
use sycamore_router::{HistoryIntegration, Route, Router};
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Route)]
enum AppRoutes {
    #[to("/")]
    HomePage,
    #[to("/owner")]
    OwnersPage,
    #[to("/add")]
    AddNewPage,
    #[not_found]
    NotFound,
}

#[component]
fn AnimalShelter<G: Html>(cx: Scope) -> View<G> {
    view! {
        cx,
        Router(
            integration=HistoryIntegration::new(),
            view=|cx, route: &ReadSignal<AppRoutes>| {
                view! { cx,
                    div(class="app") {
                        (match route.get().as_ref() {
                            AppRoutes::HomePage => view!{cx,
                                HomePage {}
                                AnimalsPage {}
                            },
                            AppRoutes::OwnersPage => view!{ cx, OwnersPage {} },
                            AppRoutes::AddNewPage => view!{ cx, AddNewPage {} },
                            AppRoutes::NotFound => view! { cx, "404 | Not found"},
                        })
                    }
                }
            }
        )
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    sycamore::render(AnimalShelter);
}
