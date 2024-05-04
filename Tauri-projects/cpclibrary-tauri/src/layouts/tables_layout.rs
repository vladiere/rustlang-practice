use leptos::*;
use leptos_router::Outlet;

#[component]
pub fn TablesLayout() -> impl IntoView {
    view! {
        <Outlet />
    }
}
