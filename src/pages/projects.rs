use leptos::prelude::*;
use leptos_router::components::A;
use crate::components::sidebar::Sidebar;
use crate::components::new_project_modal::NewProjectModal;

#[derive(Clone)]
pub struct ProjectItem {
    pub id: String,
    pub name: String,
    pub description: String,
    pub progress: u8,
    pub role: String,
}

#[component]
pub fn ProjectsPage() -> impl IntoView {
    // Controls visibility of the new project modal.
    let is_modal_open = RwSignal::new(false);

    let my_projects = vec![
        ProjectItem {
            id: "1".to_string(),
            name: "E-shop Redesign".to_string(),
            description: "Úprava frontendovej časti a optimalizácia.".to_string(),
            progress: 75,
            role: "Owner".to_string(),
        },
        ProjectItem {
            id: "3".to_string(),
            name: "Osobný Blog".to_string(),
            description: "Môj tech blog v Ruste.".to_string(),
            progress: 90,
            role: "Owner".to_string(),
        },
    ];

    let shared_projects = vec![
        ProjectItem {
            id: "2".to_string(),
            name: "Mobile App API".to_string(),
            description: "Vývoj backendu pre novú mobilnú aplikáciu.".to_string(),
            progress: 40,
            role: "Developer".to_string(),
        },
        ProjectItem {
            id: "4".to_string(),
            name: "Marketingový web".to_string(),
            description: "Landing page pre klienta.".to_string(),
            progress: 15,
            role: "Reviewer".to_string(),
        },
    ];

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
                    <h2>"My Projects"</h2>
                    <div class="projects-grid">
                        {my_projects.into_iter().map(|project| {
                            // Build the project board route dynamically from the project identifier.
                            let board_url = format!("/projects/{}/board", project.id);
                            
                            view! {
                                <A href=board_url attr:class="project-card">
                                    <div class="project-card-header">
                                        <h3>{project.name}</h3>
                                        <span class="role-badge owner">{project.role}</span>
                                    </div>
                                    <p class="project-desc">{project.description}</p>
                                    <div class="progress-bar-container">
                                        <div class="progress-bar" style=format!("width: {}%;", project.progress)></div>
                                    </div>
                                    <div class="progress-text">
                                        {project.progress} "% complete"
                                    </div>
                                </A>
                            }
                        }).collect_view()}
                    </div>
                </section>

                <section class="projects-section" style="margin-top: 3rem;">
                    <h2>"Shared with me"</h2>
                    <div class="projects-grid">
                        {shared_projects.into_iter().map(|project| {
                            // Build the project board route dynamically from the project identifier.
                            let board_url = format!("/projects/{}/board", project.id);

                            view! {
                                <A href=board_url attr:class="project-card">
                                    <div class="project-card-header">
                                        <h3>{project.name}</h3>
                                        <span class="role-badge shared">{project.role}</span>
                                    </div>
                                    <p class="project-desc">{project.description}</p>
                                    <div class="progress-bar-container">
                                        <div class="progress-bar" style=format!("width: {}%;", project.progress)></div>
                                    </div>
                                    <div class="progress-text">
                                        {project.progress} "% complete"
                                    </div>
                                </A>
                            }
                        }).collect_view()}
                    </div>
                </section>
            </main>

            <NewProjectModal is_open=is_modal_open />
        </div>
    }
}