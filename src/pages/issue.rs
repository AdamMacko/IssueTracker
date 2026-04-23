use leptos::prelude::*;
use crate::components::sidebar::Sidebar;

#[derive(Clone)]
pub enum Issue {
    Epic(Epic),
    Task(Task),
}

#[derive(Clone)]
pub struct Epic {
    pub jira_id: String,
    pub name: String,
    pub tag: String,
    pub project_name: String, // Associates the epic with its parent project for filtering.
    pub tasks: usize,
}

#[derive(Clone)]
pub struct Task {
    pub jira_id: String,
    pub name: String,
    pub tag: String,
    pub project_name: String, // Associates the task with its parent project for filtering.
}

#[component]
pub fn IssuePage() -> impl IntoView {
    // Available project filters displayed in the header tab navigation.
    let project_categories = vec![
        "All Projects", 
        "E-shop Redesign", 
        "Mobile App API", 
        "Osobný Blog"
    ];

    // Stores the currently active project filter.
    let (selected_project, set_selected_project) = signal("All Projects".to_string());

    // Demo issue dataset enriched with project ownership metadata.
    let issues = vec![
        Issue::Epic(Epic {
            jira_id: "ESHOP-1".to_string(),
            name: "Implementovať nový prihlasovací systém".to_string(),
            tag: "Backend".to_string(),
            project_name: "E-shop Redesign".to_string(),
            tasks: 3,
        }),
        Issue::Task(Task {
            jira_id: "ESHOP-3".to_string(),
            name: "Opraviť responzivitu formulára na mobiloch".to_string(),
            tag: "Frontend".to_string(),
            project_name: "E-shop Redesign".to_string(),
        }),
        Issue::Task(Task {
            jira_id: "APP-12".to_string(),
            name: "Vytvoriť API endpoint pre profil".to_string(),
            tag: "API".to_string(),
            project_name: "Mobile App API".to_string(),
        }),
        Issue::Epic(Epic {
            jira_id: "BLOG-1".to_string(),
            name: "Migrácia na Leptos 0.6".to_string(),
            tag: "Tech Debt".to_string(),
            project_name: "Osobný Blog".to_string(),
            tasks: 5,
        }),
    ];

    view! {
        <div class="dashboard-layout">
            <Sidebar />

            <main class="dashboard-content">
                <header class="dashboard-header">
                    <div style="display: flex; justify-content: space-between; align-items: flex-start;">
                        <div>
                            <h1>"Active Tasks"</h1>
                            <p>"Manage your epics, tasks, and subtasks across projects."</p>
                        </div>
                        <button class="primary-button" style="min-width: auto; height: 40px;">"Create Issue"</button>
                    </div>

                    // Horizontal project filter tabs for narrowing the issue list.
                    <div class="project-tabs">
                        {project_categories.into_iter().map(|category| {
                            let cat_clone = category.to_string();
                            let is_active = move || selected_project.get() == cat_clone;
                            let set_cat = category.to_string();
                            
                            view! {
                                <button 
                                    class="project-tab"
                                    class:active=is_active
                                    on:click=move |_| set_selected_project.set(set_cat.clone())
                                >
                                    {category}
                                </button>
                            }
                        }).collect_view()}
                    </div>
                </header>

                <section class="issues-section">
                    <div class="issue-list">
                        // Filters issues based on the currently selected project tab.
                        {move || issues.clone().into_iter()
                            .filter(|issue| {
                                let current_selected = selected_project.get();
                                if current_selected == "All Projects" {
                                    return true;
                                }
                                match issue {
                                    Issue::Epic(e) => e.project_name == current_selected,
                                    Issue::Task(t) => t.project_name == current_selected,
                                }
                            })
                            .map(|issue| {
                            match issue {
                                Issue::Epic(epic) => view! {
                                    <div class="issue-card epic-card">
                                        <div class="issue-top">
                                            <span class="issue-id">{epic.jira_id}</span>
                                            <span class="badge epic-badge">"Epic"</span>
                                        </div>
                                        <h3 class="issue-name">{epic.name}</h3>
                                        <div class="issue-bottom">
                                            <span class="tag-badge">{epic.tag}</span>
                                            <span class="task-count">
                                                {format!("{} podúloh", epic.tasks)}
                                            </span>
                                        </div>
                                    </div>
                                }.into_any(),
                                
                                Issue::Task(task) => view! {
                                    <div class="issue-card task-card">
                                        <div class="issue-top">
                                            <span class="issue-id">{task.jira_id}</span>
                                            <span class="badge task-badge">"Task"</span>
                                        </div>
                                        <h3 class="issue-name">{task.name}</h3>
                                        <div class="issue-bottom">
                                            <span class="tag-badge">{task.tag}</span>
                                        </div>
                                    </div>
                                }.into_any(),
                            }
                        }).collect_view()}
                    </div>
                </section>
            </main>
        </div>
    }
}