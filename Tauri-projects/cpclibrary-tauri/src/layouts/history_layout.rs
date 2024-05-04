use leptos::*;
use leptos_router::Outlet;

#[component]
pub fn HistoryLayout() -> impl IntoView {
    view! {
        <Outlet />
    }
}
