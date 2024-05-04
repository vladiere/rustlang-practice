use sycamore::prelude::*;

use crate::components::title::TitleTag;

#[component]
pub fn HomePage<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        div(class="w-full h-full p-2") {
            TitleTag(title="Dashboard".to_string())
            div(class="relative overflow-x-auto") {
                table(class="w-full text-sm text-left rtl:text-right text-gray-500 dark:text-gray-400") {
                    thead(class="text-xs uppercase bg-gray-700 text-gray-400") {
                        tr {
                            th(scope="col", class="px-6 py-3") { "Product name" }
                            th(scope="col", class="px-6 py-3") { "Color" }
                            th(scope="col", class="px-6 py-3") { "Category" }
                            th(scope="col", class="px-6 py-3") { "Price" }
                        }
                    }
                    tbody {
                        tr(class="border-b bg-gray-800 border-gray-700") {
                            th(scope="row", class="px-6 py-4 font-medium whitespace-nowrap text-white") { "Apple MacBook Pro 17" }
                            td(class="px-6 py-4") { "Silver" }
                            td(class="px-6 py-4") { "Laptop" }
                            td(class="px-6 py-4") { "$29999" }
                        }
                        tr(class="border-b bg-gray-800 border-gray-700") {
                            th(scope="row", class="px-6 py-4 font-medium whitespace-nowrap text-white") { "Microsoft Surface Pro" }
                            td(class="px-6 py-4") { "White" }
                            td(class="px-6 py-4") { "Laptop PC" }
                            td(class="px-6 py-4") { "$19999" }
                        }
                        tr(class="border-b bg-gray-800 border-gray-700") {
                            th(scope="row", class="px-6 py-4 font-medium whitespace-nowrap text-white") { "Magic Mouse 2" }
                            td(class="px-6 py-4") { "Black" }
                            td(class="px-6 py-4") { "Accessories" }
                            td(class="px-6 py-4") { "$99" }
                        }
                        tr(class="border-b bg-gray-800 border-gray-700") {
                            th(scope="row", class="px-6 py-4 font-medium whitespace-nowrap text-white") { "Gaming Chair" }
                            td(class="px-6 py-4") { "White and Pink" }
                            td(class="px-6 py-4") { "Chair" }
                            td(class="px-6 py-4") { "$39999" }
                        }
                    }
                }
            }
        }
    }
}
