use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Sidebar() -> impl IntoView {
    // Tracks the open state of the mobile navigation drawer.
    let (is_open, set_is_open) = signal(false);

    let toggle_menu = move |_| set_is_open.update(|open| *open = !*open);
    let close_menu = move |_| set_is_open.set(false);

    view! {
        // Mobile-only trigger for opening the sidebar.
        <button class="mobile-menu-toggle" on:click=toggle_menu>
            <span class="hamburger-icon">"☰"</span>
        </button>

        // Overlay displayed behind the sidebar while it is open on mobile.
        <div 
            class="sidebar-backdrop" 
            class:is-visible=is_open 
            on:click=close_menu
        ></div>

        <aside class="sidebar" class:is-open=is_open>
            <div class="sidebar-logo">
                <div class="logo-icon"></div>
                <h2>"Tracker"</h2>
                
                // Mobile-only close action rendered inside the sidebar.
                <button class="mobile-menu-close" on:click=close_menu>"✕"</button>
            </div>
            
            <nav class="sidebar-nav">
                // Leptos router link automatically appends the active class for the current route.
                <A href="/dashboard" attr:class="nav-link" on:click=close_menu.clone()>
                    <span class="nav-icon">"⊞"</span> "Dashboard"
                </A>
                <A href="/projects" attr:class="nav-link" on:click=close_menu.clone()>
                    <span class="nav-icon">"📁"</span> "Projects"
                </A>
                <A href="/issue" attr:class="nav-link" on:click=close_menu.clone()>
                    <span class="nav-icon">"✓"</span> "Issues"
                </A>
            </nav>

            <div class="sidebar-footer">
                <div class="user-profile">
                    <div class="avatar-sm">"JM"</div>
                    <div class="user-info">
                        <span class="user-name">"Jozef Mak"</span>
                        <a href="/login" class="logout-link">"Log out"</a>
                    </div>
                </div>
            </div>
        </aside>
    }
}