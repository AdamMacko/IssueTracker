use crate::pages::board::BoardPage;
use crate::pages::dashboard::DashboardPage;
use crate::pages::home::HomePage;
use crate::pages::issue::IssuePage;
use crate::pages::login::LoginPage;
use crate::pages::projects::ProjectsPage;
use crate::pages::register::RegisterPage;
use crate::components::protected_route::ProtectedRoute;
use crate::components::toast::{Toaster, ToastContainer};

use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title, Link};
use leptos_router::{components::{Route, Router, Routes, ParentRoute}, path};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let toaster = Toaster::new();
    provide_context(toaster);

    view! {
        <Stylesheet id="leptos" href="/pkg/issue-tracker.css"/>
        <Title text="Tracker"/>
        <Link rel="icon" type_="image/svg+xml" href="/icon.svg" />

        <ToastContainer />

        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=path!("/") view=HomePage/>
                    <Route path=path!("/login") view=LoginPage/>
                    <Route path=path!("/register") view=RegisterPage/>

                    <ParentRoute path=path!("") view=ProtectedRoute>
                        <Route path=path!("/dashboard") view=DashboardPage/>
                        <Route path=path!("/issue") view=IssuePage/>
                        <Route path=path!("/projects") view=ProjectsPage/>
                        <Route path=path!("/projects/:id/board") view=BoardPage/>
                    </ParentRoute>
                </Routes>
            </main>
        </Router>
    }
}