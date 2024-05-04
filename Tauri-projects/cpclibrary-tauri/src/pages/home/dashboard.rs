use crate::components::{
    charts::{BarChartComponent, LineChartComponent, MyData, PieChartComponent},
    dashboard::RecentActivity,
};
use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn DashboardPage() -> impl IntoView {
    let debug = RwSignal::new(false);
    let my_data = RwSignal::new(vec![
        MyData::new(5., 5., 4.5, 24.20, 20.22, 21.0),
        MyData::new(10., 3.2, 10., 20.24, 20.22, 21.0),
        MyData::new(15., 2.2, 1., 23.23, 20.22, 21.0),
        MyData::new(20., 6.2, 30., 29.22, 20.22, 21.0),
        MyData::new(25., 3.2, 12., 49.90, 20.22, 21.0),
        MyData::new(30., 8.2, 10., 22.5, 20.25, 31.0),
        MyData::new(35., 32.2, 11., 4.5, 23.22, 11.0),
        MyData::new(40., 12.2, 20.65, 44., 20.12, 51.0),
        MyData::new(45., 11.2, 32.3, 23., 41.22, 21.0),
        MyData::new(50., 22.2, 42., 4., 40.22, 11.0),
        MyData::new(55., 52.62, 22., 5.54, 45.72, 51.0),
        MyData::new(60., 62.52, 62., 6.23, 50.92, 14.0),
        MyData::new(65., 12.32, 52., 3.66, 10.62, 16.0),
    ]);

    view! {
        <Box class="bg-transparent flex flex-col">
            <h1 class="font-medium">"Dashboard"</h1>
            <Box class="grid lg:grid-cols-2 gap-5 mt-3">
                <Box class="grid sm:grid-cols-3 gap-5">
                    <Box style="background-color: var(--box-bg-color)" class="px-4 py-8 flex flex-col justify-center items-center text-center rounded-lg lg:transform hover:scale-110 shadow-md hover:shadow-lg transition-transform duration-200">
                        <Box style="background-color: var(--box-bg-color)" class="bg-neutral-500">
                            <Icon icon=icondata::LuBookUp2 style="font-size: 48px" class="text-primary-500 text-5xl leading-none" />
                            <P class="mt-2">"Unreturned Books"</P>
                            <Box style="background-color: var(--box-bg-color)" class="bg-neutral-500 text-primary-500 mt-5 text-3xl leading-none">"11"</Box>
                        </Box>
                    </Box>
                    <Box style="background-color: var(--box-bg-color)" class="px-4 py-8 flex flex-col justify-center items-center text-center bg-neutral-500 rounded-lg lg:transform hover:scale-110 shadow-md hover:shadow-lg transition-transform duration-200">
                        <Box style="background-color: var(--box-bg-color)" class="bg-neutral-500">
                            <Icon icon=icondata::BsCloudHaze2 style="font-size: 48px" class="text-primary-500 text-5xl leading-none" />
                            <P class="mt-2">"Pendings"</P>
                            <Box style="background-color: var(--box-bg-color)" class="bg-neutral-500 text-primary-500 mt-5 text-3xl leading-none">"22"</Box>
                        </Box>
                    </Box>
                    <Box style="background-color: var(--box-bg-color)" class="px-4 py-8 flex flex-col justify-center items-center text-center bg-neutral-500 rounded-lg lg:transform hover:scale-110 shadow-md hover:shadow-lg transition-transform duration-200">
                        <Box style="background-color: var(--box-bg-color)" class="bg-neutral-500">
                            <Icon icon=icondata::LuActivity style="font-size: 48px" class="text-primary-500 text-5xl leading-none" />
                            <P class="mt-2">"Active users"</P>
                            <Box style="background-color: var(--box-bg-color)" class="bg-neutral-500 text-primary-500 mt-5 text-3xl leading-none">"19"</Box>
                        </Box>
                    </Box>
                </Box>
                <Box class="grid grid-cols-2 sm:grid-cols-4 gap-5">
                    <Box style="background-color: var(--box-bg-color)" class="rounded-lg p-5 flex flex-col justify-left items-center gap-5 dark:bg-neutral-900 rounded-lg shadow-md">
                        <H6 class=" uppercase">"Total fines"</H6>
                        <small class="text-2xl mt-2 dark:text-neutral-400 flex items-center text-primary-500">
                            <Icon icon=icondata::TbCurrencyPeso style="font-size: 18px" />
                            "1,300"
                        </small>
                        <small class="uppercase">"monday"</small>
                    </Box>
                    <Box style="background-color: var(--box-bg-color)" class="rounded-lg p-5 flex flex-col justify-left items-center gap-5 dark:bg-neutral-900 rounded-lg shadow-md">
                        <H6 class=" uppercase">"Total fines"</H6>
                        <small class="text-2xl mt-2 dark:text-neutral-400 flex items-center text-primary-500">
                            <Icon icon=icondata::TbCurrencyPeso style="font-size: 18px" />
                            "1,300"
                        </small>
                        <small class="uppercase">"monday"</small>
                    </Box>
                    <Box style="background-color: var(--box-bg-color)" class="rounded-lg p-5 flex flex-col justify-left items-center gap-5 dark:bg-neutral-900 rounded-lg shadow-md">
                        <H6 class=" uppercase">"Total fines"</H6>
                        <small class="text-2xl mt-2 dark:text-neutral-400 flex items-center text-primary-500">
                            <Icon icon=icondata::TbCurrencyPeso style="font-size: 18px" />
                            "1,300"
                        </small>
                        <small class="uppercase">"monday"</small>
                    </Box>
                    <Box style="background-color: var(--box-bg-color)" class="rounded-lg p-5 flex flex-col justify-left items-center gap-5 dark:bg-neutral-900 rounded-lg shadow-md">
                        <H6 class=" uppercase">"Total fines"</H6>
                        <small class="text-2xl mt-2 dark:text-neutral-400 flex items-center text-primary-500">
                            <Icon icon=icondata::TbCurrencyPeso style="font-size: 18px" />
                            "1,300"
                        </small>
                        <small class="uppercase">"monday"</small>
                    </Box>
                </Box>
            </Box>
            <Box class="grid grid-cols-2 gap-5 mt-5">
                <LineChartComponent debug=debug.into() data=my_data.into() />
                <RecentActivity />
            </Box>
            <Box class="grid grid-cols-2 gap-5 mt-5">
                <BarChartComponent debug=debug.into() data=my_data.into() />
                <PieChartComponent />
            </Box>
        </Box>
    }
}
