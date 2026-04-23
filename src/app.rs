use crate::pages::home::HomePage;
use crate::pages::login::LoginPage;
use crate::pages::register::RegisterPage;
use crate::pages::issue::IssuePage;
use crate::pages::dashboard::DashboardPage;
use crate::pages::projects::ProjectsPage;
use crate::pages::board::BoardPage;
use crate::pages::project_details::ProjectDetailsPage;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title,Link};
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};


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

    view! {
        
        <Stylesheet id="leptos" href="/pkg/issue-tracker.css"/>
        <Title text="Issue Tracker"/>
    <Link rel="icon" type_="image/svg+xml" href="/icon.svg" />
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=path!("/") view=HomePage/>
                    <Route path=path!("/login") view=LoginPage/>
                    <Route path=path!("/register") view=RegisterPage/>
                    <Route path=path!("/dashboard") view=DashboardPage/>
                    <Route path=path!("/issue") view=IssuePage/>
                    <Route path=path!("/projects") view=ProjectsPage/>
                    <Route path=path!("/project_details") view=ProjectDetailsPage/>
                    <Route path=path!("/projects/:id/board") view=BoardPage/>
                </Routes>
            </main>
        </Router>
    }
}