use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn Notifications() -> impl IntoView {
    view! {
        <Box style="background-color: var(--box-bg-color); border-color: var(box-border-color);" class="absolute mt-5 rounded-lg border">
            <P>"This is the Notifications"</P>
        </Box>
    }
}
