use leptos::prelude::*;
use crate::components::sidebar::Sidebar;
use crate::server::tasks::{get_my_issues, TaskDto};
use crate::server::projects::{get_my_projects, ProjectMemberDto};
use crate::components::toast::Toaster;

#[component]
pub fn IssuePage() -> impl IntoView {
    let _toaster = expect_context::<Toaster>();
    let my_issues_res = Resource::new(
        || (),
        |_| async move { get_my_issues().await },
    );

    let my_projects_res = Resource::new(
        || (),
        |_| async move { get_my_projects().await },
    );

    let (selected_project_id, set_selected_project_id) = signal(0i64);

    view! {
        <div class="dashboard-layout">
            <Sidebar />

            <main class="dashboard-content">
                <header class="dashboard-header">
                    <div class="dashboard-header__top">
                        <div>
                            <h1>"My Assigned Tasks"</h1>
                            <p class="dashboard-header__subtitle">
                                "Overview of your tasks across all projects."
                            </p>
                        </div>
                    </div>

                    <div class="project-tabs">
                        <button
                            class="project-tab"
                            class:active=move || selected_project_id.get() == 0
                            on:click=move |_| set_selected_project_id.set(0)
                        >
                            "All Projects"
                        </button>

                        <Suspense fallback=move || {
                            view! {
                                <span class="project-tab project-tab--skeleton">
                                    "..."
                                </span>
                            }
                        }>
                            {move || {
                                let projects: Vec<ProjectMemberDto> = my_projects_res
                                    .get()
                                    .and_then(|res| res.ok())
                                    .unwrap_or_default();

                                projects
                                    .into_iter()
                                    .map(|p| {
                                        let p_id = p.id;
                                        let p_name = p.username.clone();

                                        view! {
                                            <button
                                                class="project-tab"
                                                class:active=move || {
                                                    selected_project_id.get() == p_id
                                                }
                                                on:click=move |_| {
                                                    set_selected_project_id.set(p_id)
                                                }
                                            >
                                                {p_name}
                                            </button>
                                        }
                                    })
                                    .collect_view()
                            }}
                        </Suspense>
                    </div>
                </header>

                <section class="issues-section">
                    <Suspense fallback=move || {
                        view! {
                            <div class="issues-loading">"Loading tasks..."</div>
                        }
                    }>
                        {move || {
                            let tasks: Vec<TaskDto> = my_issues_res
                                .get()
                                .and_then(|res| res.ok())
                                .unwrap_or_default();

                            if tasks.is_empty() {
                                return view! {
                                    <div class="issues-empty">
                                        <span class="issues-empty__icon">"📭"</span>
                                        <p>"You have no assigned tasks."</p>
                                    </div>
                                }
                                .into_any();
                            }

                            let filtered: Vec<_> = tasks
                                .into_iter()
                                .filter(|task| {
                                    let filter_id = selected_project_id.get();
                                    filter_id == 0 || task.project_id == filter_id
                                })
                                .collect();

                            if filtered.is_empty() {
                                return view! {
                                    <div class="issues-empty">
                                        <span class="issues-empty__icon">"🔍"</span>
                                        <p>"No tasks for the selected project."</p>
                                    </div>
                                }
                                .into_any();
                            }

                            view! {
                                <div class="issue-list">
                                    {filtered
                                        .into_iter()
                                        .map(|task| {
                                            let title = task.title.clone();
                                            let status = task.status.clone();
                                            let id = task.id;

                                            view! {
                                                <div class="issue-card task-card">
                                                    <div class="issue-top">
                                                        <span class="issue-id">"#" {id}</span>
                                                        <span class="badge task-badge">
                                                            {status}
                                                        </span>
                                                    </div>
                                                    <h3 class="issue-name">{title}</h3>
                                                </div>
                                            }
                                        })
                                        .collect_view()}
                                </div>
                            }
                            .into_any()
                        }}
                    </Suspense>
                </section>
            </main>
        </div>
    }
}