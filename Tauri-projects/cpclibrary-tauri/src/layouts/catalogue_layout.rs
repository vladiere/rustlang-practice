use leptos::*;
use leptos_router::Outlet;

#[component]
pub fn CatalogueLayout() -> impl IntoView {
    view! {
        <Outlet />
    }
}
