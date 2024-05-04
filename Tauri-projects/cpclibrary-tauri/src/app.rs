use leptonic::prelude::*;
use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::{
    layouts::MainLayout,
    pages::{
        AddNewResources, AppError, DashboardPage, ErrorTemplate, ListsTables, LoginPage,
        PendingTables, ProfilePage, StaffsTables, UsersTables,
    },
};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Root default_theme=LeptonicTheme::default()>
            <Router fallback=|| {
                let mut outside_errors = Errors::default();
                outside_errors.insert_with_default_key(AppError::NotFound);
                view! {
                    <ErrorTemplate outside_errors/>
                }
            }>
                <Routes>
                    <Route path="/" view=LoginPage />
                    <Route path="/home" view=MainLayout >
                        <Route path="" view=DashboardPage />
                        // <Route path="" view=move || {
                        //     view! {
                        //         <Show when=|| is_loaded() fallback=|| view! {
                        //             <LoadingState />
                        //         }>
                        //             <DashboardPage />
                        //         </Show>
                        //     }
                        // } />
                        <Route path="/profile" view=ProfilePage />
                        <Route path="/tables" view=move || {
                            view! {
                                <Outlet />
                            }
                        }>
                            <Route path="/users" view=UsersTables />
                            <Route path="/staffs" view=move || {
                                view! {
                                    <Outlet />
                                }
                            } >
                                <Route path="" view=StaffsTables />
                                // <Route path="/register" view=StaffRegister />
                            </Route>
                        </Route>
                        <Route path="/circulations" view=move || {
                            view! {
                                <Outlet />
                            }
                        } >
                            <Route path="/pending" view=PendingTables />
                        </Route>
                        <Route path="/resources" view=move || {
                            view! {
                                <Outlet />
                            }
                        } >
                            <Route path="/lists" view=ListsTables />
                            <Route path="/add" view=AddNewResources />
                        </Route>
                        <Route path="/catalogue" view=move || {
                            view! {
                                <Outlet />
                            }
                        } />
                        <Route path="/acquisitions" view=move || {
                            view! {
                                <Outlet />
                            }
                        } />
                        <Route path="/history" view=move || {
                            view! {
                                <Outlet />
                            }
                        } />
                    </Route>
                </Routes>
            </Router>
        </Root>
    }
}
