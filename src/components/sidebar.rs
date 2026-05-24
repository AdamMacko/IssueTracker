use leptos::prelude::*;
use leptos_router::components::A;
use crate::server::auth::{get_user_profile, LogoutUser};
use crate::components::toast::Toaster;

#[component]
pub fn Sidebar() -> impl IntoView {
    let toaster = expect_context::<Toaster>();
    let (is_open, set_is_open) = signal(false);

    let toggle_menu = move |_| set_is_open.update(|open| *open = !*open);
    let close_menu = move |_| set_is_open.set(false);

    let user_resource = Resource::new(|| (), |_| async move { get_user_profile().await });
    let logout_action = ServerAction::<LogoutUser>::new();
    let logout_result = logout_action.value();

    Effect::new(move |_| {
        if let Some(res) = logout_result.get() {
            match res {
                Ok(_) => {
                    toaster.success("Logged out successfully");
                }
                Err(e) => {
                    let err_msg = e
                        .to_string()
                        .replace("error running server function:", "")
                        .trim()
                        .to_string();
                    toaster.error(format!("Logout failed: {}", err_msg));
                }
            }
        }
    });

    view! {
        <button class="mobile-menu-toggle" on:click=toggle_menu>
            <span class="hamburger-icon">"☰"</span>
        </button>

        <div class="sidebar-backdrop" class:is-visible=is_open on:click=close_menu></div>

        <aside class="sidebar" class:is-open=is_open>
            <div class="sidebar-logo">
                <div class="logo-icon"></div>
                <h2>"Tracker"</h2>
                <button class="mobile-menu-close" on:click=close_menu>
                    "✕"
                </button>
            </div>

            <nav class="sidebar-nav">
                <A
                    href="/dashboard"
                    attr:class="nav-link"
                    on:click=close_menu.clone()
                >
                    <span class="nav-icon">"⊞"</span>
                    "Dashboard"
                </A>
                <A
                    href="/projects"
                    attr:class="nav-link"
                    on:click=close_menu.clone()
                >
                    <span class="nav-icon">"📁"</span>
                    "Projects"
                </A>
                <A
                    href="/issue"
                    attr:class="nav-link"
                    on:click=close_menu.clone()
                >
                    <span class="nav-icon">"✓"</span>
                    "Issues"
                </A>
            </nav>

            <div class="sidebar-footer">
                <Transition fallback=move || {
                    view! { <div class="user-profile">"Loading..."</div> }
                }>
                    {move || {
                        user_resource.get().map(|res| match res {
                            Ok(Some(user)) => {
                                let initials: String = user
                                    .username
                                    .split_whitespace()
                                    .take(2)
                                    .filter_map(|p| p.chars().next())
                                    .collect::<String>()
                                    .to_uppercase();

                                view! {
                                    <div class="user-profile">
                                        <div class="avatar-sm">{initials}</div>
                                        <div class="user-info">
                                            <span class="user-name">{user.username}</span>
                                            <ActionForm
                                                action=logout_action
                                                attr:style="margin: 0;"
                                            >
                                                <button
                                                    type="submit"
                                                    class="logout-link"
                                                    style="background: none; border: none; padding: 0; cursor: pointer; text-align: left;"
                                                >
                                                    "Log out"
                                                </button>
                                            </ActionForm>
                                        </div>
                                    </div>
                                }
                                .into_any()
                            }
                            _ => view! { <div></div> }.into_any(),
                        })
                    }}
                </Transition>
            </div>
        </aside>
    }
}