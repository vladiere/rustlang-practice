use sycamore::prelude::*;

#[component]
pub fn AddAnimal<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        form(class="w-[50%] flex flex-col mt-5 gap-5") {
            div {
                label(for="breed",class="block mb-2 text-sm font-medium text-white") { "Breed" }
                input(type="text",id="breed",class="outline-none border border-gray-300 text-sm rounded-lg block w-full p-2.5 bg-gray-700 border-gray-600 placeholder-gray-400 text-white focus:ring-blue-500 focus:border-blue-500",placeholder="e.g. German", required=true )
            }
            div {
                label(for="color", class="block mb-2 text-sm font-medium text-white") { "Color" }
                input(type="text", id="color", class="outline-none border border-gray-300 text-gray-900 text-sm rounded-lg focus:border-blue-500 block w-full p-2.5 bg-gray-700 border-gray-600 placeholder-gray-400 text-white focus:ring-blue-500 dark:focus:border-blue-500", placeholder="e.g. Brown", required=true )
            }
            div {
                label(for="name", class="block mb-2 text-sm font-medium text-white") { "Name" }
                input(type="text", id="name", class="outline-none border border-gray-300 text-gray-900 text-sm rounded-lg focus:border-blue-500 block w-full p-2.5 bg-gray-700 border-gray-600 placeholder-gray-400 text-white focus:ring-blue-500 dark:focus:border-blue-500", placeholder="e.g. John", required=true )
            }
            button(type="button", class="text-white self-start outline-none bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800") {
                "Submit"
            }
        }
    }
}
