use sycamore_router::*;

#[derive(Route)]
pub enum AppRoutes {
    #[to("/")]
    HomePage,
    #[to("/add")]
    AddNewPage,
    #[to("/owner")]
    OwnerPage,
    #[to("/animal")]
    AnimalPage,
    #[not_found]
    NotFound,
}
