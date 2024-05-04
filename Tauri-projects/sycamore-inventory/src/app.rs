use sycamore::prelude::*;
use sycamore_router::*;

use crate::{
    layouts::main_layout::MainLayout,
    pages::{add::AddNewPage, animal::AnimalPage, home::HomePage, owner::OwnerPage},
    routes::AppRoutes,
};

#[component]
pub fn App<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        Router(
            integration=HistoryIntegration::new(),
            view=|cx, route: &ReadSignal<AppRoutes>| {
                view! { cx,
                    div(class="h-screen bg-slate-900 text-white") {
                        (match route.get().as_ref() {
                            AppRoutes::HomePage => view! { cx,
                                div(class="flex flex-row") {
                                    MainLayout {}
                                    HomePage {}
                                }
                            },
                            AppRoutes::AddNewPage => view! { cx,
                                div(class="flex flex-row") {
                                    MainLayout {}
                                    AddNewPage {}
                                }
                            },
                            AppRoutes::OwnerPage => view! { cx,
                                div(class="flex flex-row") {
                                    MainLayout {}
                                    OwnerPage {}
                                }
                            },
                            AppRoutes::AnimalPage => view! { cx,
                                div(class="flex flex-row") {
                                    MainLayout {}
                                    AnimalPage {}
                                }
                            },
                            AppRoutes::NotFound => view! { cx, "404 | Not Found" },
                        })
                    }
                }
            }
        )
    }
}
