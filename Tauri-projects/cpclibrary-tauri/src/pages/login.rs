use leptonic::prelude::*;
use leptos::leptos_dom::ev::SubmitEvent;
use leptos::*;
use leptos_dom::logging::console_log;

use crate::services::auth::{login_handler, LoginCmdArgs, LoginCmdArgsWrapper};

#[component]
pub fn LoginPage() -> impl IntoView {
    let (email, set_email) = create_signal(String::new());
    let (pass, set_pass) = create_signal(String::new());

    let update_email = move |ev| {
        let v = event_target_value(&ev);
        set_email.set(v);
    };

    let update_pass = move |ev| {
        let v = event_target_value(&ev);
        set_pass.set(v);
    };

    let submit_login = move |ev: SubmitEvent| {
        ev.prevent_default();

        spawn_local(async move {
            let email = email.get_untracked();
            let pass = pass.get_untracked();

            if email.is_empty() || pass.is_empty() {
                return;
            }

            let data = LoginCmdArgsWrapper {
                payload: LoginCmdArgs {
                    email_address: email,
                    password: pass,
                },
            };

            let result = login_handler(data).await;

            console_log(result.as_str());
        });
    };

    view! {
        <Box class="h-screen text-gray-900 flex items-center bg-center bg-cover bg-no-repeat" style="background:url('public/backgrounds/bg2.jpg')">
            <div class="w-screen h-screen bg-black opacity-35 absolute z-10">
            </div>
            <div class="z-30 h-full max-w-7xl mx-auto px-4 sm:px-6 lg:px-4 flex items-center justify-center">
                <div class="lg:w-3/6 lg:pr-0 pr-0">
                    <H1 class="font-extrabold text-5xl text-white flex gap-10 items-center">
                        <img class="w-20 h-20 mr-2" src="public/applogo.png" alt="logo" />
                        "CPC Library"
                    </H1>
                    <P class="leading-relaxed mt-4 text-white text-2xl">"Lorem ipsum dolor sit amet, consectetur adipiscing elit,sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."</P>
                </div>
                <form on:submit=submit_login class="lg:w-3/6 xl:w-2/5 md:w-full bg-gray-50 p-8 flex flex-col lg:ml-auto w-full mt-10 lg:mt-0 rounded-md">
                    <H1 class="text-2xl font-medium">"Sign in to you account"</H1>
                    <div class="relative mb-4">
                        <label for="email" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">Your email</label>
                        <input type="email" name="email" id="email" class="bg-gray-50 border border-gray-300 text-gray-900 sm:text-sm rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="name@company.com" required=true on:input=update_email />
                    </div>
                    <div class="relative mb-4">
                        <label for="password" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">Password</label>
                        <input type="password" name="password" id="password" placeholder="••••••••" class="bg-gray-50 border border-gray-300 text-gray-900 sm:text-sm rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" required=true on:input=update_pass />
                    </div>
                    <button type="submit" class="text-white text-center bg-blue-700 hover:bg-blue-800 hover:text-white focus:ring-4 focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 dark:bg-blue-600 dark:hover:bg-blue-700 focus:outline-none dark:focus:ring-blue-800" >"Sign in"</button>
                </form>
            </div>
        </Box>
    }
}
