use leptos::prelude::*;
use crate::server::auth::LoginUser;
use crate::components::toast::Toaster;

#[component]
pub fn LoginPage() -> impl IntoView {
    let login_action = ServerAction::<LoginUser>::new();
    let is_pending = login_action.pending();
    let result = login_action.value();
    let toaster = expect_context::<Toaster>();

    Effect::new(move |_| {
        if let Some(res) = result.get() {
            match res {
                Ok(_) => {
                    toaster.success("Successfully signed in! Redirecting...");
                }
                Err(e) => {
                    let err_msg = e
                        .to_string()
                        .replace("error running server function:", "")
                        .trim()
                        .to_string();
                    toaster.error(format!("Login failed: {}", err_msg));
                }
            }
        }
    });

    view! {
        <div class="auth-layout">
            <div class="auth-card">
                <div class="auth-header">
                    <div class="logo-icon-large"></div>
                    <h1>"Welcome back"</h1>
                    <p>"Enter your details to access your workspace."</p>
                </div>

                <ActionForm action=login_action attr:class="auth-form">
                    <div class="input-group">
                        <label for="email">"Email"</label>
                        <input
                            type="email"
                            id="email"
                            name="email"
                            placeholder="name@company.com"
                            required
                        />
                    </div>

                    <div class="input-group">
                        <div style="display: flex; justify-content: space-between;">
                            <label for="password">"Password"</label>
                            <a href="#" class="forgot-link">"Forgot password?"</a>
                        </div>
                        <input
                            type="password"
                            id="password"
                            name="password"
                            placeholder="••••••••"
                            required
                        />
                    </div>

                    <button
                        type="submit"
                        class="primary-button auth-submit"
                        disabled=is_pending
                    >
                        {move || {
                            if is_pending.get() {
                                "Signing in..."
                            } else {
                                "Sign in"
                            }
                        }}
                    </button>
                </ActionForm>

                <p class="auth-redirect" style="margin-top: 2rem;">
                    "Don't have an account? "
                    <a href="/register">"Sign up"</a>
                </p>
            </div>
        </div>
    }
}