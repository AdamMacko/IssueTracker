use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use crate::components::sidebar::Sidebar;
use crate::components::comments::IssueDiscussion;
use crate::components::time_tracker::TimeTracker;

#[derive(Clone)]
pub struct KanbanTask {
    pub id: String,
    pub title: String,
    pub status: String,
    pub issue_type: String,
    pub assignee: String,
}

#[component]
pub fn BoardPage() -> impl IntoView {
    let params = use_params_map();
    let project_id = move || params.with(|p| p.get("id").unwrap_or_default());

    // Tracks the currently selected task for the slide-over detail panel.
    let (selected_task_id, set_selected_task_id) = signal::<Option<String>>(None);

    let close_slide_over = move |_| set_selected_task_id.set(None);

    let tasks = vec![
        KanbanTask { id: "ESHOP-15".into(), title: "Opraviť košík na mobile".into(), status: "To Do".into(), issue_type: "Bug".into(), assignee: "Jozef Mak".into() },
        KanbanTask { id: "ESHOP-18".into(), title: "Pripraviť podklady pre marketing".into(), status: "To Do".into(), issue_type: "Task".into(), assignee: "Jana Nováková".into() },
        KanbanTask { id: "ESHOP-12".into(), title: "Stripe platobná brána".into(), status: "In Progress".into(), issue_type: "Epic".into(), assignee: "Peter Hraško".into() },
        KanbanTask { id: "ESHOP-9".into(), title: "Aktualizácia React knižníc".into(), status: "In Review".into(), issue_type: "Task".into(), assignee: "Jozef Mak".into() },
        KanbanTask { id: "ESHOP-4".into(), title: "Nasadiť Google Analytics".into(), status: "Done".into(), issue_type: "Task".into(), assignee: "Jana Nováková".into() },
    ];

    let get_tasks = move |status_filter: &str| {
        tasks.iter().filter(|t| t.status == status_filter).cloned().collect::<Vec<_>>()
    };

    view! {
        <div class="dashboard-layout">
            <Sidebar />

            <main class="dashboard-content">
                <header class="dashboard-header">
                    <div style="display: flex; justify-content: space-between; align-items: flex-start;">
                        <div>
                            <div class="breadcrumbs">
                                <a href="/projects">"Projects"</a>
                                <span class="separator">"/"</span>
                                <span>"Project #" {project_id}</span>
                            </div>
                            <h1 style="margin-top: 0.25rem;">"Kanban Board"</h1>
                        </div>
                        <div style="display: flex; gap: 1rem;">
                            <div class="board-team">
                                <div class="avatar-micro" title="Jozef Mak">"JM"</div>
                                <div class="avatar-micro" title="Jana Nováková" style="background: #f59e0b;">"JN"</div>
                                <div class="avatar-micro" title="Peter Hraško" style="background: #10b981;">"PH"</div>
                            </div>
                            <button class="secondary-button" style="height: 36px;">"Share"</button>
                        </div>
                    </div>
                </header>

                <section class="kanban-wrapper">
                    <div class="kanban-board">
                        {["To Do", "In Progress", "In Review", "Done"].into_iter().map(|col_name| {
                            let col_tasks = get_tasks(col_name);
                            let count = col_tasks.len();
                            
                            view! {
                                <div class="kanban-column">
                                    <div class="column-header">
                                        <h3>{col_name} <span class="task-count">{count}</span></h3>
                                        <button class="add-task-btn">"+"</button>
                                    </div>
                                    
                                    <div class="column-cards">
                                        {col_tasks.into_iter().map(|task| {
                                            let initial = task.assignee.chars().next().unwrap_or('?').to_string();
                                            let type_color = if task.issue_type == "Bug" { "red" } else if task.issue_type == "Epic" { "purple" } else { "blue" };
                                            let task_id_clone = task.id.clone();

                                            view! {
                                                // Opens the task detail panel for the selected card.
                                                <div 
                                                    class="kanban-card" 
                                                    draggable="true"
                                                    on:click=move |_| set_selected_task_id.set(Some(task_id_clone.clone()))
                                                >
                                                    <div class="card-labels">
                                                        <span class=format!("card-type {}", type_color)>{task.issue_type}</span>
                                                    </div>
                                                    <p class="card-title">{task.title}</p>
                                                    <div class="card-footer">
                                                        <span class="card-id">{task.id}</span>
                                                        <div class="avatar-micro">{initial}</div>
                                                    </div>
                                                </div>
                                            }
                                        }).collect_view()}
                                    </div>
                                </div>
                            }
                        }).collect_view()}
                    </div>
                </section>
            </main>

            <Show when=move || selected_task_id.get().is_some()>
                // Backdrop click closes the slide-over panel.
                <div class="slide-over-backdrop" on:click=close_slide_over.clone()></div>
                
                <div class="slide-over-panel">
                    <div class="slide-over-header">
                        <h2>{move || selected_task_id.get().unwrap_or_default()}</h2>
                        <button class="close-btn" on:click=close_slide_over.clone()>"✕"</button>
                    </div>
                    
                    <div class="slide-over-content">
                        <TimeTracker />

                        <p style="margin-bottom: 2rem; color: #64748b;">"Here will be the ticket description, assignee, priority, etc."</p>
                        
                        <IssueDiscussion />
                    </div>
                </div>
            </Show>

        </div>
    }
}