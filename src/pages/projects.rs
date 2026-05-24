use leptos::prelude::*;
use leptos_router::components::A;
use crate::components::sidebar::Sidebar;
use crate::components::new_project_modal::NewProjectModal;
use crate::server::projects::{get_projects, Project};
use crate::components::toast::Toaster;

#[component]
pub fn ProjectsPage() -> impl IntoView {
    let _toaster = expect_context::<Toaster>();
    let is_modal_open = RwSignal::new(false);
    let (reload_trigger, set_reload_trigger) = signal(0);

    let projects_resource = Resource::new(
        move || reload_trigger.get(),
        |_| async move { get_projects().await },
    );

    let on_project_created = Callback::new(move |_| {
        set_reload_trigger.update(|n| *n += 1);
    });

    view! {
        <div class="dashboard-layout">
            <Sidebar />

            <main class="dashboard-content">
                <header class="dashboard-header">
                    <div style="display: flex; justify-content: space-between; align-items: center;">
                        <div>
                            <h1>"Projects"</h1>
                            <p>"Manage all your personal and shared projects."</p>
                        </div>
                        <button
                            class="primary-button"
                            style="min-width: auto; height: 40px;"
                            on:click=move |_| is_modal_open.set(true)
                        >
                            "+ New Project"
                        </button>
                    </div>
                </header>

                <section class="projects-section">
                    <Transition fallback=move || {
                        view! { <div class="loading">"Loading projects..."</div> }
                    }>
                        {move || {
                            projects_resource.get().map(|res| {
                                match res {
                                    Ok(data) if data.is_empty() => {
                                        view! {
                                            <div class="empty-state">
                                                "No projects found."
                                            </div>
                                        }
                                        .into_any()
                                    }
                                    Ok(data) => {
                                        view! {
                                            <div class="projects-grid">
                                                {data
                                                    .into_iter()
                                                    .map(|project| {
                                                        view! {
                                                            <ProjectCard project />
                                                        }
                                                    })
                                                    .collect_view()}
                                            </div>
                                        }
                                        .into_any()
                                    }
                                    Err(_) => {
                                        view! {
                                            <div class="error">
                                                "Failed to load projects."
                                            </div>
                                        }
                                        .into_any()
                                    }
                                }
                            })
                        }}
                    </Transition>
                </section>
            </main>

            <NewProjectModal is_open=is_modal_open on_success=on_project_created />
        </div>
    }
}

#[component]
fn ProjectCard(project: Project) -> impl IntoView {
    let progress = 0;
    let role = "Owner";
    let board_url = format!("/projects/{}/board", project.id);

    view! {
        <A href=board_url attr:class="project-card">
            <div class="project-card-header">
                <h3>{project.name.clone()}</h3>
                <span class="role-badge owner">{role}</span>
            </div>
            <p class="project-desc">{project.description.clone()}</p>
            <div class="progress-bar-container">
                <div
                    class="progress-bar"
                    style=format!("width: {}%;", progress)
                ></div>
            </div>
            <div class="progress-text">
                {progress} "% complete"
            </div>
        </A>
    }
}