use leptonic::prelude::*;
use leptos::*;

#[derive(Clone)]
struct Minion {
    id: u32,
    name: String,
    appearance: String,
    num_eyes: u32,
}

#[component]
pub fn UsersTables() -> impl IntoView {
    let (text, set_text) = create_signal(String::new());
    let minions = create_rw_signal(vec![
        Minion {
            id: 1,
            name: String::from("Kevin"),
            appearance: String::from("Tall"),
            num_eyes: 2,
        },
        Minion {
            id: 2,
            name: String::from("Bob"),
            appearance: String::from("Short"),
            num_eyes: 2,
        },
        Minion {
            id: 3,
            name: String::from("Stuart"),
            appearance: String::from("Medium"),
            num_eyes: 1,
        },
        Minion {
            id: 4,
            name: String::from("Otto"),
            appearance: String::from("Round"),
            num_eyes: 2,
        },
    ]);

    view! {
       <Box class="bg-transparent flex flex-col gap-10">
           <Box class="flex justify-between bg-transparent">
               <H1 class="font-semibold">"Users table list"</H1>
                <div class="relative">
                    <TextInput get=text set=set_text class="w-full h-10 pl-10 pr-5 text-sm rounded-full appearance-none focus:outline-none border border-neutral-300 dark:border-neutral-700" placeholder="Search..."  />
                    <button class="absolute top-0 left-0 mt-3 ml-4">
                        <Icon icon=icondata::FaMagnifyingGlassSolid class="text-neutral-400" ></Icon>
                    </button>
                </div>
           </Box>
            <TableContainer>
                <Table bordered=true hoverable=true>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell min_width=true>"#"</TableHeaderCell>
                            <TableHeaderCell>"Name"</TableHeaderCell>
                            <TableHeaderCell>"Appearance"</TableHeaderCell>
                            <TableHeaderCell>"Num. eyes"</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <For
                            each=move || minions.get()
                            key=move |minion| minion.id
                            children=move |minion| view! {
                                <TableRow>
                                    <TableCell>{minion.id}</TableCell>
                                    <TableCell>{minion.name}</TableCell>
                                    <TableCell>{minion.appearance}</TableCell>
                                    <TableCell>{minion.num_eyes}</TableCell>
                                </TableRow>
                            }
                        />
                    </TableBody>
                </Table>
            </TableContainer>
        </Box>
    }
}
