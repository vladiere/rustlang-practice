use sycamore::prelude::*;

#[component]
pub fn AddNewPage<G: Html>(cx: Scope) -> View<G> {
    view! {
        cx,
        div(class="p-2 h-screen") {
            div(class="w-screen flex flex-row") {
                // Left corner
                div(class="w-full flex flex-col items-center") {
                    h1(class="text-2xl font-medium pb-2 border-b-2 border-slate-900 dark:border-slate-200") {
                        "Add new Animal"
                    }

                    div(class="w-[45%] flex flex-col mt-5 gap-5")
                    {
                        div {

                            label(for="first_name",class="block mb-2 text-sm font-medium text-gray-900 dark:text-white") {
                                "First name"
                            }

                            input(type="text",id="first_name",class="bg-gray-50 outline-none border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",placeholder="John", required=true )
                        }

                        div {

                            label(for="first_name", class="block mb-2 text-sm font-medium text-gray-900 dark:text-white") {
                                "First name"
                            }

                            input(type="text", id="first_name", class="bg-gray-50 outline-none border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500", placeholder="John", required=true )
                        }

                        div {

                            label(for="first_name", class="block mb-2 text-sm font-medium text-gray-900 dark:text-white") {
                                "First name"
                            }

                            input(type="text", id="first_name", class="bg-gray-50 outline-none border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500", placeholder="John", required=true )
                        }

                        button(type="button", class="text-white self-start outline-none bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800") {
                            "Submit"
                        }
                    }
                }
                // Right Corner
                div(class="w-full flex flex-col items-center") {
                    h1(class="text-2xl font-medium pb-2 border-b-2 border-slate-900 dark:border-slate-200") {
                        "Add new Owner"
                    }
                    div(class="w-[45%] flex flex-col mt-5 gap-5") {
                        div {
                            label(for="first_name", class="block mb-2 text-sm font-medium text-gray-900 dark:text-white") {
                                "Owner name"
                            }
                            input(type="text", id="first_name", class="bg-gray-50 outline-none border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500", placeholder="John", required=true )
                        }
                        button(type="button", class="text-white self-start outline-none bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800") {
                            "Submit"
                        }
                    }
                }
            }
        }
    }
}
