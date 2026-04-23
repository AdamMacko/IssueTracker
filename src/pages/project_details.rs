use leptos::prelude::*;
use crate::components::sidebar::Sidebar;

#[derive(Clone, PartialEq, Eq)]
pub enum TaskStatus {
    Todo,
    InProgress,
    InReview,
    Done,
}

#[derive(Clone)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub description: String,
    pub status: TaskStatus,
    pub assignee: String,
}

// Renders a single Kanban column with its associated task list.
#[component]
fn KanbanColumn(title: String, tasks: Vec<Task>) -> impl IntoView {
    let count = tasks.len();

    view! {
        <div class="kanban-column">
            <div class="kanban-column-header">
                <h3>{title}</h3>
                <span class="task-count">{count}</span>
            </div>
            <div class="kanban-tasks">
                {tasks.into_iter().map(|task| {
                    // Use the assignee's first character as a lightweight avatar fallback.
                    let initial = task.assignee.chars().next().unwrap_or('?').to_string();
                    
                    view! {
                        <div class="task-card">
                            <h4 class="task-title">{task.title}</h4>
                            <p class="task-desc">{task.description}</p>
                            <div class="task-footer">
                                <span class="task-id">"#" {task.id}</span>
                                <span class="task-assignee">
                                    <div class="avatar">{initial}</div>
                                    {task.assignee}
                                </span>
                            </div>
                        </div>
                    }
                }).collect_view()}
            </div>
        </div>
    }
}

#[component]
pub fn ProjectDetailsPage() -> impl IntoView {
    // In production, project data would typically be resolved from route params and fetched from the backend.
    let project_name = "E-shop Redesign";

    let tasks = vec![
        Task {
            id: "101".to_string(),
            title: "Návrh novej hlavičky".to_string(),
            description: "Vytvoriť responzívny dizajn pre navigáciu.".to_string(),
            status: TaskStatus::Done,
            assignee: "Jozef".to_string(),
        },
        Task {
            id: "102".to_string(),
            title: "Optimalizácia obrázkov".to_string(),
            description: "Nasadit WebP formát pre produktové fotky.".to_string(),
            status: TaskStatus::InReview,
            assignee: "Mária".to_string(),
        },
        Task {
            id: "103".to_string(),
            title: "Migrácia na Leptos".to_string(),
            description: "Prepísať checkout proces do Rustu.".to_string(),
            status: TaskStatus::InProgress,
            assignee: "Ty".to_string(),
        },
        Task {
            id: "104".to_string(),
            title: "Pripraviť E2E testy".to_string(),
            description: "Napísať testy pre košík.".to_string(),
            status: TaskStatus::Todo,
            assignee: "Peter".to_string(),
        },
        Task {
            id: "105".to_string(),
            title: "Aktualizácia závislostí".to_string(),
            description: "Updatnúť Cargo.toml na najnovšie verzie.".to_string(),
            status: TaskStatus::Todo,
            assignee: "Ty".to_string(),
        },
    ];

    // Precompute task groups by status for column rendering.
    let todo_tasks: Vec<_> = tasks.iter().filter(|t| t.status == TaskStatus::Todo).cloned().collect();
    let in_progress_tasks: Vec<_> = tasks.iter().filter(|t| t.status == TaskStatus::InProgress).cloned().collect();
    let in_review_tasks: Vec<_> = tasks.iter().filter(|t| t.status == TaskStatus::InReview).cloned().collect();
    let done_tasks: Vec<_> = tasks.iter().filter(|t| t.status == TaskStatus::Done).cloned().collect();

    view! {
        <div class="dashboard-layout">
            <Sidebar />

            <main class="dashboard-content">
                <header class="dashboard-header">
                    <div style="display: flex; justify-content: space-between; align-items: flex-start;">
                        <div>
                            <a href="/projects" class="back-link">"← Back to Projects"</a>
                            <h1 style="margin-top: 0.5rem;">{project_name}</h1>
                        </div>
                        <button class="primary-button" style="min-width: auto; height: 40px;">"+ Create Task"</button>
                    </div>
                </header>

                <section class="kanban-board">
                    <KanbanColumn title="To Do".to_string() tasks=todo_tasks />
                    <KanbanColumn title="In Progress".to_string() tasks=in_progress_tasks />
                    <KanbanColumn title="In Review".to_string() tasks=in_review_tasks />
                    <KanbanColumn title="Done".to_string() tasks=done_tasks />
                </section>
            </main>
        </div>
    }
}