use leptos::prelude::*;

#[derive(Clone, PartialEq)]
struct DbUser {
    id: String,
    name: String,
    email: String,
}

#[component]
pub fn NewProjectModal(
    is_open: RwSignal<bool>,
) -> impl IntoView {
    // Mock user dataset representing assignable project members.
    let available_users = vec![
        DbUser { id: "u1".to_string(), name: "Jana Nováková".to_string(), email: "jana@company.com".to_string() },
        DbUser { id: "u2".to_string(), name: "Peter Hraško".to_string(), email: "peter@company.com".to_string() },
        DbUser { id: "u3".to_string(), name: "Martin Kováč".to_string(), email: "martin@company.com".to_string() },
    ];

    let (selected_users, set_selected_users) = signal(Vec::<String>::new());

    // Toggles a user in the current project member selection.
    let toggle_user = move |user_id: String| {
        set_selected_users.update(|users| {
            if let Some(pos) = users.iter().position(|id| id == &user_id) {
                users.remove(pos);
            } else {
                users.push(user_id);
            }
        });
    };

    let close_modal = move |_ev: leptos::ev::MouseEvent| {
        is_open.set(false);
        set_selected_users.set(Vec::new()); 
    };

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        
        // TODO: Persist project data and handle member invitations.
        is_open.set(false);
        set_selected_users.set(Vec::new());
    };

    view! {
        <Show when=move || is_open.get()>
            <div class="modal-backdrop" on:click=close_modal.clone()>
                
                <div class="modal-container" on:click=move |ev| ev.stop_propagation()>
                    
                    <div class="modal-header">
                        <h2>"Create New Project"</h2>
                        <button class="close-btn" on:click=close_modal.clone()>"✕"</button>
                    </div>

                    <form class="modal-body" on:submit=on_submit>
                        <div class="form-group">
                            <label for="project-name">"Project Name"</label>
                            <input id="project-name" type="text" placeholder="e.g. E-shop Redesign" required />
                        </div>

                        <div class="form-group" style="display: flex; gap: 1rem;">
                            <div style="flex: 1;">
                                <label for="project-key">"Project Key"</label>
                                <input id="project-key" type="text" placeholder="e.g. ESHOP" style="width: 100%;" required />
                            </div>
                        </div>

                        <div class="form-group">
                            <label for="project-desc">"Description (Optional)"</label>
                            <textarea id="project-desc" rows="2" placeholder="Briefly describe the project..."></textarea>
                        </div>

                        <div class="form-group">
                            <label>"Invite Team Members"</label>
                            <div class="users-list">
                                {available_users.clone().into_iter().map(|user| {
                                    let user_id_class = user.id.clone();
                                    let user_id_show = user.id.clone();
                                    let user_id_click = user.id.clone();
                                    
                                    // Reactive class binding for selected user state.
                                    let is_selected_class = move || selected_users.with(|users| users.contains(&user_id_class));
                                    
                                    // Controls visibility of the selection indicator.
                                    let is_selected_show = move || selected_users.with(|users| users.contains(&user_id_show));
                                    
                                    // Derive avatar fallback from the user's first name character.
                                    let initial = user.name.chars().next().unwrap_or('?').to_string();

                                    view! {
                                        <div 
                                            class="user-select-item" 
                                            class:selected=is_selected_class
                                            on:click=move |_| toggle_user(user_id_click.clone())
                                        >
                                            <div class="user-avatar">{initial}</div>
                                            <div class="user-details">
                                                <span class="user-name">{user.name}</span>
                                                <span class="user-email">{user.email}</span>
                                            </div>
                                            <div class="checkbox-circle">
                                                <Show when=is_selected_show fallback=|| view! { <span>""</span> }>
                                                    "✓"
                                                </Show>
                                            </div>
                                        </div>
                                    }
                                }).collect_view()}
                            </div>
                        </div>

                        <div class="modal-footer">
                            <button type="button" class="secondary-button" on:click=close_modal.clone()>"Cancel"</button>
                            <button type="submit" class="primary-button">"Create Project"</button>
                        </div>
                    </form>

                </div>
            </div>
        </Show>
    }
}