use leptos::prelude::*;
use crate::server::tasks::CreateTask;
use crate::components::toast::Toaster;

#[component]
pub fn NewTaskModal(
    is_open: RwSignal<bool>,
    #[prop(into)] project_id: Signal<i64>,
    #[prop(into)] on_success: Callback<()>,
) -> impl IntoView {
    let toaster = expect_context::<Toaster>();
    let create_action = ServerAction::<CreateTask>::new();
    let create_result = create_action.value();

    Effect::new(move |_| {
        if let Some(res) = create_result.get() {
            match res {
                Ok(_) => {
                    toaster.success("Task created successfully");
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
                    toaster.error(format!("Failed to create task: {}", err_msg));
                }
            }
        }
    });

    let close_modal = move |_ev| {
        is_open.set(false);
    };

    view! {
        <Show when=move || is_open.get()>
            <div class="modal-backdrop" on:click=close_modal.clone()>
                <div class="modal-container" on:click=move |ev| ev.stop_propagation()>
                    <div class="modal-header">
                        <h2>"Create New Task"</h2>
                        <button class="close-btn" on:click=close_modal.clone()>
                            "✕"
                        </button>
                    </div>

                    <ActionForm action=create_action attr:class="modal-body">
                        <input
                            type="hidden"
                            name="project_id"
                            value=move || project_id.get()
                        />

                        <div class="form-group">
                            <label for="task-title">"Task Title"</label>
                            <input
                                id="task-title"
                                name="title"
                                type="text"
                                placeholder="e.g. Fix login bug"
                                required
                            />
                        </div>

                        <div class="form-group">
                            <label for="task-desc">"Description"</label>
                            <textarea
                                id="task-desc"
                                name="description"
                                rows="3"
                                placeholder="Provide more details..."
                            ></textarea>
                        </div>

                        <div class="form-group">
                            <label for="task-status">"Status"</label>
                            <select
                                id="task-status"
                                name="status"
                                style="width: 100%; padding: 0.5rem; border: 1px solid #ddd; border-radius: 4px;"
                            >
                                <option value="Todo">"To Do"</option>
                                <option value="InProgress">"In Progress"</option>
                                <option value="InReview">"In Review"</option>
                                <option value="Done">"Done"</option>
                            </select>
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
                                "Create Task"
                            </button>
                        </div>
                    </ActionForm>
                </div>
            </div>
        </Show>
    }
}