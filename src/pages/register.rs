use leptos::prelude::*;

#[component]
pub fn RegisterPage() -> impl IntoView {
    view! {
        <div class="auth-layout">
            <div class="auth-card">
                <div class="auth-header">
                    <div class="logo-icon-large"></div>
                    <h1>"Create an account"</h1>
                    <p>"Sign up to start managing your projects."</p>
                </div>

                <form class="auth-form" on:submit=|e| e.prevent_default()>
                    <div class="input-group">
                        <label for="name">"Full Name"</label>
                        <input id="name" type="text" placeholder="John Doe" />
                    </div>

                    <div class="input-group">
                        <label for="email">"Email"</label>
                        <input id="email" type="email" placeholder="name@company.com" />
                    </div>

                    <div class="input-group">
                        <label for="password">"Password"</label>
                        <input id="password" type="password" placeholder="••••••••" />
                    </div>

                    <button type="submit" class="primary-button auth-submit">"Sign up"</button>
                </form>

                <p class="auth-redirect">
                    "Already have an account? "
                    <a href="/login">"Log in"</a>
                </p>
            </div>
        </div>
    }
}