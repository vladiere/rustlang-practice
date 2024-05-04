use leptonic::prelude::*;
use leptos::*;
use leptos_meta::Style;

#[component]
pub fn RecentActivity() -> impl IntoView {
    view! {

        <Style>
            "
                ._border-table-color {
                    border-color: var(--chart-fill-color);
                    padding-top: 1.25rem;
                    padding-bottom: 1.25rem;
                }
            "
        </Style>

        <Box style="background-color: var(--box-bg-color);" class="flex flex-col gap-5 p-5 rounded-lg">
            <H3>"Recent Activity"</H3>

            <table class="w-full text-md text-left rtl:text-right">
                <thead class="text-lg text-primary-500 uppercase">
                    <tr>
                        <th class="text-left rtl:text-right py-4">"Book Title"</th>
                        <th class="w-1/5 py-4">"Name"</th>
                        <th class="w-2/12 py-4">"Date"</th>
                    </tr>
                </thead>
                <tbody>
                    <tr style="padding-top: 1.25rem !important; padding-bottom: 1.25rem !important;" class="border-t _border-table-color">
                        <th>"Lorem ipsum dolor sit amet, qui minim labore adipisicing."</th>
                        <td>"Dochs, Mohdl"</td>
                        <td>"3-15-2024"</td>
                    </tr>
                    <tr class="border-t py-4 _border-table-color">
                        <th>"Sint cillum sint consectetur cupidatat."</th>
                        <td>"Hosi, Soej"</td>
                        <td>"3-9-2024"</td>
                    </tr>
                    <tr class="border-t py-4 _border-table-color">
                        <th> "Lorem ipsum dolor sit amet, qui minim labore." </th>
                        <td>"Losk, Ashopo"</td>
                        <td>"3-5-2024"</td>
                    </tr>
                    <tr class="border-t py-4 _border-table-color">
                        <th>"Lorem ipsum dolor sit amet, qui minim labore."</th>
                        <td>"Losk, Ashopo"</td>
                        <td>"3-5-2024"</td>
                    </tr>
                    <tr class="border-t py-4 _border-table-color">
                        <th>"Lorem ipsum dolor sit amet, qui minim labore."</th>
                        <td>"Losk, Ashopo"</td>
                        <td>"3-5-2024"</td>
                    </tr>
                </tbody>
            </table>

            <Box style="background-color: var(--box-bg-color);" class="mt-auto grid grid-cols-3">
                <LinkButton href="/home" class="rounded-lg mt-5">"Show all activities"</LinkButton>
            </Box>
        </Box>
    }
}
