use leptonic::prelude::*;
use leptos::*;
use leptos_router::{Outlet, A};

#[component]
pub fn MainLayout() -> impl IntoView {
    let shown = RwSignal::new(true);

    view! {
        <Box class="h-screen w-screen flex">
            <Drawer side=DrawerSide::Left shown=shown class="h-full overflow-y-scroll px-5 py-2">
                <H1 class="font-bold text-2xl flex gap-2 items-center my-3">
                    <img class="w-10 h-10 mr-2" src="public/applogo.png" alt="logo" />
                    "CPC Library"
                </H1>

                <Separator />

                <Stack spacing=Size::Em(0.5) class="my-3">
                    <A href="/home" class="flex items-center w-full p-3  text-start leading-tight transition-all outline-none hover:border-l-2 border-blue-500 hover:rounded-r-lg">
                        <div class="grid place-items-center mr-4">
                            <Icon icon=icondata::FaHouseSolid style="font-size: 24px" />
                        </div>
                        "Dashboard"
                    </A>
                    <A href="/home/profile" class="flex items-center w-full p-3  text-start leading-tight transition-all outline-none hover:border-l-2 border-blue-500 hover:rounded-r-lg">
                        <div class="grid place-items-center mr-4">
                            <Icon icon=icondata::RiContactsUserFacesFill style="font-size: 24px" />
                        </div>
                        "Profile"
                    </A>
                    // {(0..20).map(|_| view! { <Skeleton height=Size::Em(3.0)/> }).collect_view()}
                </Stack>

                <Separator />

                <Collapsibles default_on_open=OnOpen::CloseOthers>
                    <Stack spacing=Size::Em(0.6) class="mt-3">
                        <Collapsible>
                            <CollapsibleHeader slot class="hover:border-l-2 px-3 hover:border-primary-500 w-full flex items-center gap-2">
                                <Icon icon=icondata::RiTableDesignFill style="font-size: 24px" />
                                <H4>"Tables"</H4>
                            </CollapsibleHeader>
                            <CollapsibleBody slot>
                                <Box class="bg-transparent w-full flex flex-col gap-5 items-center justify-right">
                                    <Box class="w-3/4 bg-transparent flex flex-col gap-5">
                                        <A href="/home/tables/users" class="w-full flex justify-left gap-5 items-center ml-4">
                                            <Icon icon=icondata::FaUsersSolid style="font-size: 24px" />
                                            "Users"
                                        </A>
                                        <A href="/home/tables/staffs" class="w-full flex justify-left gap-5 items-center ml-4">
                                            <Icon icon=icondata::FaUsersGearSolid style="font-size: 24px" />
                                            "Staff"
                                        </A>
                                    </Box>
                                </Box>
                            </CollapsibleBody>
                        </Collapsible>
                        <Collapsible>
                            <CollapsibleHeader slot class="hover:border-l-2 px-3 hover:border-primary-500 w-full flex items-center gap-2">
                                <Icon icon=icondata::FaRecycleSolid style="font-size: 24px" />
                                <H4>"Circulations"</H4>
                            </CollapsibleHeader>
                            <CollapsibleBody slot>
                                <Box class="bg-transparent w-full flex flex-col gap-5 items-center justify-right">
                                    <Box class="w-3/4 bg-transparent flex flex-col gap-5">
                                        <A href="/home/circulations/borrows" class="w-full flex justify-left gap-5 items-center">
                                            <Icon icon=icondata::BiCalendarEventRegular style="font-size: 24px" />
                                            "Borrows"
                                        </A>
                                        <A href="/home/checkouts" class="w-full flex justify-left gap-5 items-center">
                                            <Icon icon=icondata::BiCartDownloadRegular style="font-size: 24px" />
                                            "Checkouts"
                                        </A>
                                        <A href="/home/circulations/reserved" class="w-full flex justify-left gap-5 items-center">
                                            <Icon icon=icondata::RiReservedOthersLine style="font-size: 24px" />
                                            "Reserved"
                                        </A>
                                        <A href="/home/circulations/return" class="w-full flex justify-left gap-5 items-center">
                                            <Icon icon=icondata::TbTruckReturn style="font-size: 24px" />
                                            "Returned"
                                        </A>
                                        <A href="/home/circulations/pending" class="w-full flex justify-left gap-5 items-center">
                                            <Icon icon=icondata::BsClock style="font-size: 24px" />
                                            "Pending"
                                        </A>
                                    </Box>
                                </Box>
                            </CollapsibleBody>
                        </Collapsible>
                        <Collapsible>
                            <CollapsibleHeader slot class="hover:border-l-2 px-3 hover:border-primary-500 w-full flex items-center gap-2">
                                <Icon icon=icondata::SiBookstack style="font-size: 24px" />
                                <H4>"Resources"</H4>
                            </CollapsibleHeader>
                            <CollapsibleBody slot>
                                <Box class="bg-transparent w-full flex flex-col gap-5 items-center justify-right">
                                    <Box class="w-3/4 bg-transparent flex flex-col gap-5">
                                        <A href="/home/resources/add" class="w-full flex justify-left gap-5 items-center">
                                            <Icon icon=icondata::BiBookmarkPlusSolid style="font-size: 24px" />
                                            "Add new"
                                        </A>
                                        <A href="/home/resources/lists" class="w-full flex justify-left gap-5 items-center">
                                            <Icon icon=icondata::RiBookletDocumentFill style="font-size: 24px" />
                                            "Lists"
                                        </A>
                                    </Box>
                                </Box>
                            </CollapsibleBody>
                        </Collapsible>
                        <Collapsible>
                            <CollapsibleHeader slot class="hover:border-l-2 px-3 hover:border-primary-500 w-full flex items-center gap-2">
                                <Icon icon=icondata::FaSwatchbookSolid style="font-size: 24px" />
                                <H4>"Catalogue"</H4>
                            </CollapsibleHeader>
                            <CollapsibleBody slot>
                                <Box class="bg-transparent w-full flex flex-col gap-5 items-center justify-right">
                                    <Box class="w-3/4 bg-transparent flex flex-col gap-5">
                                        <A href="#" class="w-full flex justify-left gap-5 items-center">
                                            <Icon icon=icondata::TbPlaylistAdd style="font-size: 24px" />
                                            "Add new"
                                        </A>
                                        <A href="#" class="w-full flex justify-left gap-5 items-center">
                                            <Icon icon=icondata::SiReaddotcv style="font-size: 24px" />
                                            "Lists"
                                        </A>
                                    </Box>
                                </Box>
                            </CollapsibleBody>
                        </Collapsible>
                        <Collapsible>
                            <CollapsibleHeader slot class="hover:border-l-2 px-3 hover:border-primary-500 w-full flex items-center gap-2">
                                <Icon icon=icondata::ImCart style="font-size: 24px" />
                                <H4>"Acquisitions"</H4>
                            </CollapsibleHeader>
                            <CollapsibleBody slot>
                                <Box class="bg-transparent w-full flex flex-col gap-5 items-center justify-right">
                                    <Box class="w-3/4 bg-transparent flex flex-col gap-5">
                                        <A href="#" class="w-full flex justify-left gap-5 items-center">
                                            <Icon icon=icondata::BiArchiveInSolid style="font-size: 24px" />
                                            "Purchased"
                                        </A>
                                        <A href="#" class="w-full flex justify-left gap-5 items-center">
                                            <Icon icon=icondata::BiListOlRegular style="font-size: 24px" />
                                            "Lists"
                                        </A>
                                    </Box>
                                </Box>
                            </CollapsibleBody>
                        </Collapsible>
                        <Collapsible>
                            <CollapsibleHeader slot class="hover:border-l-2 px-3 hover:border-primary-500 w-full flex items-center gap-2">
                                <Icon icon=icondata::OcHistorySm style="font-size: 24px" />
                                <H4>"History"</H4>
                            </CollapsibleHeader>
                            <CollapsibleBody slot>
                                <Box class="bg-transparent w-full flex flex-col gap-5 items-center justify-right">
                                    <Box class="w-3/4 bg-transparent flex flex-col gap-5">
                                        <A href="#" class="w-full flex justify-left gap-5 items-center">
                                            <Icon icon=icondata::OcLogSm style="font-size: 24px" />
                                            "Logs"
                                        </A>
                                        <A href="#" class="w-full flex justify-left gap-5 items-center">
                                            <Icon icon=icondata::TbWriting style="font-size: 24px" />
                                            "Activities"
                                        </A>
                                    </Box>
                                </Box>
                            </CollapsibleBody>
                        </Collapsible>
                    </Stack>
                </Collapsibles>

            </Drawer>
            <Box class="w-full flex flex-col">
                <AppBar height=Height::Em(5.0) class="text-gray-900 p-5 flex items-center">
                    <Stack orientation=StackOrientation::Horizontal spacing=Size::Em(1.0) style="margin-right: 1em">
                        <Button variant=ButtonVariant::Flat on_click=move |_| shown.update(|show: &mut bool| *show = !(*show))>
                            <Icon icon=icondata::CgMenu style="font-size: 24px" />
                        </Button>
                    </Stack>
                    <span class="m-auto"></span>
                    <Stack orientation=StackOrientation::Horizontal spacing=Size::Em(1.0) style="margin-right: 1em">
                        <ThemeToggle off=LeptonicTheme::Light on=LeptonicTheme::Dark />
                        <Button variant=ButtonVariant::Flat on_click=move |_| {}>
                            <Icon icon=icondata::BsBell style="font-size: 18px" />
                        </Button>
                        <Button variant=ButtonVariant::Flat on_click=move |_| {}>
                            <Icon icon=icondata::ChCog style="font-size: 18px;" />
                        </Button>
                        <Button variant=ButtonVariant::Flat on_click=move |_| {}>
                            <Icon icon=icondata::AiPoweroffOutlined style="font-size: 18px" />
                        </Button>
                    </Stack>
                </AppBar>
                <Box class="overflow-y-scroll p-5 dark:bg-neutral-700 dark:text-neutral-900">
                    <Outlet />
                </Box>
            </Box>
        </Box>
    }
}
