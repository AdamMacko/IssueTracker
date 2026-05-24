use leptos::prelude::*;
use crate::server::projects::CreateProject;
use crate::server::auth::search_users;
use crate::components::toast::Toaster;

#[component]
pub fn NewProjectModal(
    is_open: RwSignal<bool>,
    #[prop(into)] on_success: Callback<()>,
) -> impl IntoView {
    let toaster = expect_context::<Toaster>();
    let create_action = ServerAction::<CreateProject>::new();
    let create_result = create_action.value();

    Effect::new(move |_| {
        if let Some(res) = create_result.get() {
            match res {
                Ok(_) => {
                    toaster.success("Project created successfully");
                    is_open.set(false);
                    create_action.clear();
                    on_success.run(());
                }
                Err(e) => {
                    let err_msg = e
                        .to_string()
                        .replace("error running server function:", "")
                        .trim()
                        .to_string();
                    toaster.error(format!("Failed to create project: {}", err_msg));
                }
            }
        }
    });

    let (search_query, set_search_query) = signal(String::new());
    let (selected_users, set_selected_users) = signal(Vec::<i64>::new());

    let users_resource = Resource::new(
        move || search_query.get(),
        |query| async move { search_users(query).await },
    );

    let toggle_user = move |user_id: i64| {
        set_selected_users.update(|users| {
            if let Some(pos) = users.iter().position(|id| id == &user_id) {
                users.remove(pos);
            } else {
                users.push(user_id);
            }
        });
    };

    let close_modal = move |_ev| {
        is_open.set(false);
        set_selected_users.set(Vec::new());
        set_search_query.set(String::new());
    };

    view! {
        <Show when=move || is_open.get()>
            <div class="modal-backdrop" on:click=close_modal.clone()>
                <div class="modal-container" on:click=move |ev| ev.stop_propagation()>
                    <div class="modal-header">
                        <h2>"Create New Project"</h2>
                        <button class="close-btn" on:click=close_modal.clone()>
                            "✕"
                        </button>
                    </div>

                    <ActionForm action=create_action attr:class="modal-body">
                        <div class="form-group">
                            <label for="project-name">"Project Name"</label>
                            <input
                                id="project-name"
                                name="name"
                                type="text"
                                placeholder="e.g. E-shop Redesign"
                                required
                            />
                        </div>

                        <div class="form-group" style="display: flex; gap: 1rem;">
                            <div style="flex: 1;">
                                <label for="project-key">"Project Key"</label>
                                <input
                                    id="project-key"
                                    name="project_key"
                                    type="text"
                                    placeholder="e.g. ESHOP"
                                    style="width: 100%;"
                                    required
                                />
                            </div>
                        </div>

                        <div class="form-group">
                            <label for="project-desc">"Description (Optional)"</label>
                            <textarea
                                id="project-desc"
                                name="description"
                                rows="2"
                                placeholder="Briefly describe the project..."
                            ></textarea>
                        </div>

                        <input
                            type="hidden"
                            name="invited_users_str"
                            value=move || {
                                selected_users
                                    .get()
                                    .iter()
                                    .map(|id| id.to_string())
                                    .collect::<Vec<_>>()
                                    .join(",")
                            }
                        />

                        <div class="form-group">
                            <label>"Invite Team Members"</label>

                            <input
                                type="text"
                                placeholder="Search by name or email..."
                                style="margin-bottom: 0.5rem; width: 100%; padding: 0.5rem; border: 1px solid #ddd; border-radius: 4px;"
                                on:input=move |ev| set_search_query.set(event_target_value(&ev))
                                prop:value=search_query
                            />

                            <div class="users-list">
                                <Transition fallback=move || {
                                    view! {
                                        <div style="text-align: center; color: #666; padding: 1rem;">
                                            "Searching..."
                                        </div>
                                    }
                                }>
                                    {move || {
                                        users_resource.get().map(|res| match res {
                                            Ok(users) if users.is_empty() => {
                                                view! {
                                                    <div style="text-align: center; color: #666; padding: 1rem;">
                                                        "No users found."
                                                    </div>
                                                }
                                                .into_any()
                                            }
                                            Ok(users) => {
                                                view! {
                                                    <>
                                                        {users
                                                            .into_iter()
                                                            .map(|user| {
                                                                let uid = user.id;
                                                                let is_selected = move || {
                                                                    selected_users.with(|u| u.contains(&uid))
                                                                };
                                                                let initial = user
                                                                    .username
                                                                    .chars()
                                                                    .next()
                                                                    .unwrap_or('?')
                                                                    .to_string()
                                                                    .to_uppercase();

                                                                view! {
                                                                    <div
                                                                        class="user-select-item"
                                                                        class:selected=is_selected
                                                                        on:click=move |_| toggle_user(uid)
                                                                    >
                                                                        <div class="user-avatar">
                                                                            {initial}
                                                                        </div>
                                                                        <div class="user-details">
                                                                            <span class="user-name">
                                                                                {user.username.clone()}
                                                                            </span>
                                                                            <span class="user-email">
                                                                                {user.email.clone()}
                                                                            </span>
                                                                        </div>
                                                                        <div class="checkbox-circle">
                                                                            <Show
                                                                                when=is_selected
                                                                                fallback=|| {
                                                                                    view! { <span>""</span> }
                                                                                }
                                                                            >
                                                                                "✓"
                                                                            </Show>
                                                                        </div>
                                                                    </div>
                                                                }
                                                            })
                                                            .collect_view()}
                                                    </>
                                                }
                                                .into_any()
                                            }
                                            Err(_) => {
                                                view! {
                                                    <div style="color: red;">
                                                        "Database connection error."
                                                    </div>
                                                }
                                                .into_any()
                                            }
                                        })
                                    }}
                                </Transition>
                            </div>
                        </div>

                        <div class="modal-footer">
                            <button
                                type="button"
                                class="secondary-button"
                                on:click=close_modal.clone()
                            >
                                "Cancel"
                            </button>
                            <button
                                type="submit"
                                class="primary-button"
                                disabled=create_action.pending()
                            >
                                "Create Project"
                            </button>
                        </div>
                    </ActionForm>
                </div>
            </div>
        </Show>
    }
}