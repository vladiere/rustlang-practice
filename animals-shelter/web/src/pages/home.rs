use sycamore::prelude::*;

#[component]
pub fn HomePage<G: Html>(cx: Scope) -> View<G> {
    view! {
        cx,
        div(class="h-screen flex flex-col bg-slate-200 dark:bg-slate-900 dark:text-white") {
            div(class="w-full py-5 flex flex-row items-center justify-center gap-x-5 border-b-[1px] border-slate-900 dark:border-slate-200") {
                a(href="/", class="hover:text-blue-500") {
                    "Home"
                }
                a(href="/owner", class="hover:text-blue-500") {
                    "Owners"
                }
                a(href="/add", class="hover:text-blue-500") {
                    "Add new"
                }
            }
        }
    }
}
