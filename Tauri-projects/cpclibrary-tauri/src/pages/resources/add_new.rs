use leptonic::prelude::*;
use leptos::*;
use time::OffsetDateTime;

#[component]
pub fn AddNewResources() -> impl IntoView {
    view! {
        <Box class="bg-transparent flex flex-col gap-5">
            <H1 class="font-semibold">"Add new book"</H1>
            <H4 class="font-medium">
                <span class="text-red-500">"NOTE: "</span>
                "We require a minimum set of fields to create a new record. These are
    those fields."
            </H4>
            <Box class="bg-transparent grid grid-cols-2 gap-5">
                <div>
                    <label for="book_title" class="block mb-2 text-sm font-medium">"Book title"</label>
                    <input type="text" name="book_title" id="file_img" class="bg-[var(--basic-input-background-color)] border border-gray-300 text-gray-900 sm:text-sm rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5 " placeholder="Enter title of the bookd" requied=true />
                </div>
                <div>
                    <label for="accession_no" class="block mb-2 text-sm font-medium">"Accession Number *"</label>
                    <input type="text" name="accession_no" id="file_img" class="bg-[var(--basic-input-background-color)] border border-gray-300 text-gray-900 sm:text-sm rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5 " placeholder="Enter book accession no" requied=true />
                </div>
                <div>
                    <label for="author" class="block mb-2 text-sm font-medium">"Book author"</label>
                    <input type="text" name="author" id="author" class="bg-[var(--basic-input-background-color)] border border-gray-300 text-gray-900 sm:text-sm rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5 " placeholder="Enter book author" requied=true />
                </div>
                <div>
                    <label for="book_edition" class="block mb-2 text-sm font-medium">"Book edition"</label>
                    <input type="text" name="book_edition" id="file_img" class="bg-[var(--basic-input-background-color)] border border-gray-300 text-gray-900 sm:text-sm rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5 " placeholder="Enter book edition" requied=true />
                </div>
                <div>
                    <label for="book_publisher" class="block mb-2 text-sm font-medium">"Publisher"</label>
                    <input type="text" name="book_publisher" id="file_img" class="bg-[var(--basic-input-background-color)] border border-gray-300 text-gray-900 sm:text-sm rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5 " placeholder="Enter publisher name" requied=true />
                </div>
                <div>
                    <label for="copyright_yr" class="block mb-2 text-sm font-medium">"Copyright Year"</label>
                    <input type="number" name="copyright_yr" id="file_img" class="bg-[var(--basic-input-background-color)] border border-gray-300 text-gray-900 sm:text-sm rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5 " placeholder="Enter copyright year" requied=true />
                </div>
                <div>
                    <label for="cost_price" class="block mb-2 text-sm font-medium">"Cost value"</label>
                    <input type="number" name="cost_price" id="file_img" class="bg-[var(--basic-input-background-color)] border border-gray-300 text-gray-900 sm:text-sm rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5 " placeholder="Enter cost value" requied=true />
                </div>

                <div>
                    <DateSelector value=OffsetDateTime::now_utc() on_change=move |_v| {}/>
                </div>
            </Box>
            <div>
                <label for="message" class="block mb-2 text-sm font-medium">"Book remarks (Empty if none)"</label>
                <textarea id="message" rows="4" class="block p-2.5 w-full text-sm text-gray-900 bg-[var(--basic-input-background-color)] rounded-lg border border-gray-300 focus:ring-blue-500 focus:border-blue-500" placeholder="Write your book remarks here..."></textarea>
            </div>
            <Box class="bg-transparent flex items-center justify-center w-full">
                <label for="dropzone-file" class="flex flex-col items-center justify-center w-full h-64 border-2 border-gray-300 border-dashed rounded-lg cursor-pointer bg-[var(--basic-input-background-color)] dark:hover:bg-bray-800 dark:bg-gray-700 hover:bg-gray-100 dark:border-gray-600 dark:hover:border-gray-500 dark:hover:bg-gray-600">
                    <div class="flex flex-col items-center justify-center pt-5 pb-6">
                        <svg class="w-8 h-8 mb-4 text-gray-500 dark:text-gray-400" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 20 16">
                            <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 13h3a3 3 0 0 0 0-6h-.025A5.56 5.56 0 0 0 16 6.5 5.5 5.5 0 0 0 5.207 5.021C5.137 5.017 5.071 5 5 5a4 4 0 0 0 0 8h2.167M10 15V6m0 0L8 8m2-2 2 2"/>
                        </svg>
                        <p class="mb-2 text-sm text-gray-500 dark:text-gray-400"><span class="font-semibold">"Click to upload"</span> "or drag and drop"</p>
                        <p class="text-xs text-gray-500 dark:text-gray-400">"SVG, PNG, JPG or GIF (MAX. 800x400px)"</p>
                    </div>
                    <input id="dropzone-file" type="file" class="hidden" />
                </label>
            </Box>
        </Box>
    }
}
