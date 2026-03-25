use leptos::prelude::*;
use leptos_router::hooks::use_navigate; // Potrebujeme na presmerovanie

#[component]
pub fn LoginPage() -> impl IntoView {
    // Vytvoríme stav (state) pre email a heslo
    let (email, set_email) = signal(String::new());
    let (password, set_password) = signal(String::new());

    // Pripravíme si funkciu na presmerovanie
    let navigate = use_navigate();

    // Funkcia, ktorá sa spustí pri odoslaní formulára
    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default(); // Zabráni klasickému znovunačítaniu stránky prehliadačom

        // Overíme, či sú hodnoty "admin"
        if email.get() == "admin" && password.get() == "admin" {
            // Presmerujeme používateľa na novú stránku (napr. /admin alebo /dashboard)
            navigate("/admin", Default::default());
        } else {
            // Ak zadal zlé údaje, môžeme mu to zatiaľ aspoň vypísať do konzoly
            leptos::logging::log!("Nesprávne meno alebo heslo!");
            // Neskôr tu môžeš nastaviť napr. zobrazenie chybovej hlášky priamo na obrazovke
        }
    };

    view! {
        <section class="login-page">
            <section class="hero">
                <h1>"Login"</h1>
                <p>"Welcome back! Please enter your details."</p>
            </section>
            
            // Formuláru pridáme našu on_submit funkciu
            <form on:submit=on_submit>
                <div>
                    <label for="email">"Email"</label>
                    <input 
                        id="email" 
                        type="email" 
                        placeholder="Enter email"
                        prop:value=email
                        on:input=move |ev| set_email.set(event_target_value(&ev))
                    />
                </div>

                <div>
                    <label for="password">"Password"</label>
                    <input 
                        id="password" 
                        type="password" 
                        placeholder="Enter password"
                        // Prepojíme input s naším signálom
                        prop:value=password
                        on:input=move |ev| set_password.set(event_target_value(&ev))
                    />
                </div>

                <button type="submit">"Log in"</button>
                
                <div style="margin-top: 1.5rem; text-align: center; font-size: 0.875rem;">
                    <span style="color: #9aa4b2;">"Don't have an account? "</span>
                    <a href="/register" style="color: #4f8cff; font-weight: 600;">"Register"</a>
                </div>
            </form>
        </section>
    }
}