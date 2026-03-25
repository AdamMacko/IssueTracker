use leptos::prelude::*;

#[component]
pub fn RegisterPage() -> impl IntoView {
    view! {
        <section class="register-page">
            <section class="hero">
                <h1>"Register"</h1>
                <p>"Create a new account"</p>
            </section>
            
            <form>
                <div>
                    <label for="name">"Name"</label>
                    <input id="name" type="text" placeholder="Enter name" />
                </div>

                <div>
                    <label for="email">"Email"</label>
                    <input id="email" type="email" placeholder="Enter email" />
                </div>

                <div>
                    <label for="password">"Password"</label>
                    <input id="password" type="password" placeholder="Enter password" />
                </div>

                <button type="submit">"Sign up"</button>

                
                <div style="margin-top: 1.5rem; text-align: center; font-size: 0.875rem;">
                    <span style="color: #9aa4b2;">"Already have an account? "</span>
                    <a href="/login" style="color: #4f8cff; font-weight: 600;">"Log in"</a>
                </div>
            </form>
        </section>
    }
}