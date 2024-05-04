use sycamore::prelude::*;

#[component(inline_props)]
pub fn Dropdown<'a, G: Html>(cx: Scope<'a>, value: &'a ReadSignal<bool>) -> View<G> {
    view! { cx,
        (if *value.get() {
            view! { cx,
                div(id="dropdown", class="mt-2 z-10 bg-white divide-y divide-gray-100 rounded-lg shadow w-44 dark:bg-gray-700") {
                    ul(class="py-2 text-sm text-gray-700 dark:text-gray-200", aria-labelledby="dropdownDefaultButton") {
                        li {
                            a(href="#", class="block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white") {
                                "Dashboard"
                            }
                        }
                        li {
                            a(href="#", class="block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white") {
                                "Settings"
                            }
                        }
                        li {
                            a(href="#", class="block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white") {
                                "Earnings"
                            }
                        }
                        li {
                            a(href="#", class="block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white") {
                                "Sign out"
                            }
                        }
                    }
                }
            }
        } else { view! { cx, } })
    }
}
