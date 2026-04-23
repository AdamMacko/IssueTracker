use leptos::prelude::*;

#[component]
pub fn LoginPage() -> impl IntoView {
    view! {
        <div class="auth-layout">
            <div class="auth-card">
                <div class="auth-header">
                    <div class="logo-icon-large"></div>
                    <h1>"Welcome back"</h1>
                    <p>"Enter your details to access your workspace."</p>
                </div>

                <form class="auth-form" on:submit=|e| e.prevent_default()>
                    <div class="input-group">
                        <label for="email">"Email"</label>
                        <input type="email" id="email" placeholder="name@company.com" />
                    </div>
                    
                    <div class="input-group">
                        <div style="display: flex; justify-content: space-between;">
                            <label for="password">"Password"</label>
                            <a href="#" class="forgot-link">"Forgot password?"</a>
                        </div>
                        <input type="password" id="password" placeholder="••••••••" />
                    </div>

                    <button type="submit" class="primary-button auth-submit">"Sign in"</button>
                </form>

                <p class="auth-redirect">
                    "Don't have an account? "
                    <a href="/register">"Sign up"</a>
                </p>
            </div>
        </div>
    }
}