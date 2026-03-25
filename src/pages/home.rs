use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="landing-page">
            <header class="landing-header">
                <div class="brand">"Issue Tracker"</div>

                <nav class="landing-nav">
                    <A href="/login" attr:class="nav-link">
                        "Sign in"
                    </A>
                    <A href="/register" attr:class="nav-button">
                        "Sign up"
                    </A>
                </nav>
            </header>

            <section class="hero">
                <p class="hero-badge">"Project management for modern teams"</p>

                <h1>"Manage projects, tasks, and teamwork in one place"</h1>

                <p class="hero-text">
                    "Issue Tracker helps you organize work, track progress, collaborate with teammates, "
                    "and keep every project under control."
                </p>

                <div class="hero-actions">
                    <A href="/register" attr:class="primary-button">
                        "Get started"
                    </A>
                    <A href="/login" attr:class="secondary-button">
                        "Sign in"
                    </A>
                </div>
            </section>

            <section class="features">
                <div class="feature-card">
                    <h2>"Project organization"</h2>
                    <p>"Keep projects, milestones, and tasks structured in one shared workspace."</p>
                </div>

                <div class="feature-card">
                    <h2>"Team collaboration"</h2>
                    <p>"Invite colleagues, assign work, comment on issues, and stay aligned."</p>
                </div>

                <div class="feature-card">
                    <h2>"Progress tracking"</h2>
                    <p>"Track task status, time spent, and project progress from start to finish."</p>
                </div>
            </section>
        </div>
    }
}