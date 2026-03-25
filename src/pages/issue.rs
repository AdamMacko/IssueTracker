use leptos::prelude::*;

// 1. Upravené modely pre použitie v UI
#[derive(Clone)]
pub enum Issue {
    Epic(Epic),
    Task(Task),
    Subtask(Task), // Predpokladám, že subtask má podobnú štruktúru ako task
}

#[derive(Clone)]
pub struct Epic {
    pub jira_id: String, // V Ruste je štandardom snake_case namiesto camelCase
    pub name: String,
    pub tag: String,
    pub tasks: Vec<Task>,
}

#[derive(Clone)]
pub struct Task {
    pub jira_id: String,
    pub name: String,
    pub tag: String,
}

#[component]
pub fn IssuePage() -> impl IntoView {
    // 2. Vytvoríme si testovacie dáta (simulácia toho, čo by prišlo z databázy)
    let issues = vec![
        Issue::Epic(Epic {
            jira_id: "PROJ-1".to_string(),
            name: "Implementovať nový prihlasovací systém".to_string(),
            tag: "Backend".to_string(),
            tasks: vec![
                Task {
                    jira_id: "PROJ-2".to_string(),
                    name: "Vytvoriť databázovú tabuľku".to_string(),
                    tag: "Database".to_string(),
                }
            ],
        }),
        Issue::Task(Task {
            jira_id: "PROJ-3".to_string(),
            name: "Opraviť responzivitu formulára na mobiloch".to_string(),
            tag: "Frontend".to_string(),
        }),
    ];

    // 3. Vykreslenie stránky
    view! {
        <div class="issue-page">
            <header class="issue-header">
                <h1>"Active Tasks"</h1>
                <button class="primary-button">"Create Issue"</button>
            </header>

            <div class="issue-list">
                // Prejdeme všetky issues a podľa typu (Epic/Task) vykreslíme správny dizajn
                {issues.into_iter().map(|issue| {
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
                                        {format!("{} podúloh", epic.tasks.len())}
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

                        Issue::Subtask(_) => view! { <div></div> }.into_any(),
                    }
                }).collect_view()}
            </div>
        </div>
    }
}