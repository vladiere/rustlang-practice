use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn ProfilePage() -> impl IntoView {
    view! {
        <Box class="bg-transparent flex flex-col shadow-lg rounded-lg">
            <Box class="bg-transparent overflow-hidden h-1/4">
                <img class="w-full h-full" src="public/backgrounds/bg.jpg" alt="background library" />
            </Box>
            <Box class="flex justify-center px-5 -mt-12">
                <img class="w-32 h-32 rounded-full border-3 border-[var(--box-border-color)]" src="https://images.pexels.com/photos/19224969/pexels-photo-19224969/free-photo-of-college-girl-sitting-in-library-reading.jpeg?auto=compress&cs=tinysrgb&w=600" alt="librarian picture" />
            </Box>
            <Box class="text-center px-14">
                <H2 class="uppercase text-3xl font-bold">"descartin, lance phillip b."</H2>
                <P class="mt-2 capitalize">"@Admin"</P>
                <P class="mt-2 text-sm">"Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat."</P>
            </Box>
        </Box>
    }
}
