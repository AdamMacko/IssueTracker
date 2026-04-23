use leptos::prelude::*;
use crate::components::sidebar::Sidebar;

#[derive(Clone, Debug, PartialEq)]
struct TeamMemberStats {
    name: String,
    hours: f32,
    color: String,
}

#[component]
pub fn DashboardPage() -> impl IntoView {
    // Tracks the currently selected project for the analytics view.
    let (selected_project, set_selected_project) = signal("E-shop Mobile App".to_string());

    // Memoized dataset that updates whenever the selected project changes.
    let stats = Memo::new(move |_| {
        if selected_project.get() == "E-shop Mobile App" {
            vec![
                TeamMemberStats { name: "Jozef Mak".into(), hours: 42.5, color: "#6366F1".into() },
                TeamMemberStats { name: "Jana Nováková".into(), hours: 28.0, color: "#F59E0B".into() },
                TeamMemberStats { name: "Peter Hraško".into(), hours: 15.5, color: "#10B981".into() },
            ]
        } else {
            vec![
                TeamMemberStats { name: "Jozef Mak".into(), hours: 10.0, color: "#6366F1".into() },
                TeamMemberStats { name: "Jana Nováková".into(), hours: 55.2, color: "#F59E0B".into() },
            ]
        }
    });

    view! {
        <div class="dashboard-layout">
            <Sidebar />
            
            <main class="dashboard-content">
                <header class="dashboard-header" style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 2rem;">
                    <div>
                        <h1>"Team Analytics"</h1>
                        <p style="color: #64748b;">"Overview of time distribution across the team."</p>
                    </div>
                    
                    <select 
                        class="project-select"
                        on:change=move |ev| set_selected_project.set(event_target_value(&ev))
                    >
                        <option value="E-shop Mobile App">"E-shop Mobile App"</option>
                        <option value="Marketing Website">"Marketing Website"</option>
                    </select>
                </header>

                <div class="stats-grid">
                    <div class="stat-card">
                        <span class="label">"Total Time Tracked"</span>
                        <span class="value">{move || format!("{:.1} h", stats.get().iter().map(|s| s.hours).sum::<f32>())}</span>
                    </div>
                    <div class="stat-card">
                        <span class="label">"Active Members"</span>
                        <span class="value">{move || stats.get().len()}</span>
                    </div>
                </div>

                <div class="chart-container">
                    <h3>"Hours worked per member"</h3>
                    <div class="bar-chart">
                        {move || stats.get().into_iter().map(|member| {
                            // Scale each bar proportionally using 60 hours as the visual maximum.
                            let bar_width = format!("{}%", (member.hours / 60.0 * 100.0).min(100.0));
                            view! {
                                <div class="chart-row">
                                    <span class="member-name">{member.name}</span>
                                    <div class="bar-wrapper">
                                        <div 
                                            class="bar" 
                                            style=format!("width: {}; background-color: {};", bar_width, member.color)
                                        ></div>
                                        <span class="bar-value">{member.hours} "h"</span>
                                    </div>
                                </div>
                            }
                        }).collect_view()}
                    </div>
                </div>
            </main>
        </div>
    }
}