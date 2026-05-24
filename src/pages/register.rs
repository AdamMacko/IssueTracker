// Importujeme základné veci z Leptosu, napríklad komponenty,
// signály, Effect, ActionForm a view! makro.
use leptos::prelude::*;

// Serverová akcia, ktorá rieši registráciu nového používateľa.
use crate::server::auth::RegisterUser;

// Toaster používame na zobrazovanie hlášok používateľovi,
// napríklad úspešná registrácia alebo chyba.
use crate::components::toast::Toaster;

// Toto je stránka pre registráciu nového používateľa.
#[component]
pub fn RegisterPage() -> impl IntoView {
    // Z contextu si vytiahneme toaster na zobrazovanie notifikácií.
    let toaster = expect_context::<Toaster>();

    // ServerAction slúži na odoslanie formulára na server.
    // Tu konkrétne posiela meno, email a heslo do RegisterUser.
    let register_action = ServerAction::<RegisterUser>::new();

    // result obsahuje výsledok serverovej akcie.
    // Po odoslaní formulára tu dostaneme buď Ok alebo Err.
    let result = register_action.value();

    // is_pending hovorí, či sa formulár práve odosiela.
    // Používame to na vypnutie tlačidla počas registrácie.
    let is_pending = register_action.pending();

    // Effect sleduje výsledok register akcie.
    // Keď server odpovie, zobrazíme úspech alebo chybu.
    Effect::new(move |_| {
        if let Some(res) = result.get() {
            match res {
                Ok(_) => {
                    // Ak sa registrácia podarila, zobrazíme úspešnú hlášku.
                    // Samotné presmerovanie pravdepodobne rieši serverová akcia alebo router.
                    toaster.success("Registration successful! Redirecting to login...");
                }
                Err(e) => {
                    // Chybovú správu zo servera prevedieme na čitateľnejší text.
                    let err_msg = e
                        .to_string()
                        .replace("error running server function:", "")
                        .trim()
                        .to_string();

                    // Zobrazíme chybu používateľovi.
                    toaster.error(format!("Registration failed: {}", err_msg));
                }
            }
        }
    });

    view! {
        // Hlavný layout registračnej stránky.
        <div class="auth-layout">
            // Karta s registračným formulárom.
            <div class="auth-card">
                <div class="auth-header">
                    // Väčšia ikona loga.
                    <div class="logo-icon-large"></div>

                    // Nadpis stránky.
                    <h1>"Create an account"</h1>

                    // Krátky text pod nadpisom.
                    <p>"Sign up to start managing your projects."</p>
                </div>

                // Formulár napojený na register_action.
                // Po submitnutí sa dáta pošlú do serverovej akcie RegisterUser.
                <ActionForm action=register_action attr:class="auth-form">
                    <div class="input-group">
                        // Label pre celé meno používateľa.
                        <label for="name">"Full Name"</label>

                        // Input pre meno používateľa.
                        // name="username" znamená, že server očakáva pole username.
                        <input
                            id="name"
                            name="username"
                            type="text"
                            placeholder="John Doe"
                            required
                        />
                    </div>

                    <div class="input-group">
                        // Label pre email.
                        <label for="email">"Email"</label>

                        // Input pre email používateľa.
                        // type="email" pomáha prehliadaču validovať formát emailu.
                        <input
                            id="email"
                            name="email"
                            type="email"
                            placeholder="name@company.com"
                            required
                        />
                    </div>

                    <div class="input-group">
                        // Label pre heslo.
                        <label for="password">"Password"</label>

                        // Input pre heslo.
                        // type="password" skryje zadávaný text.
                        <input
                            id="password"
                            name="password"
                            type="password"
                            placeholder="••••••••"
                            required
                        />
                    </div>

                    // Submit tlačidlo pre registráciu.
                    // Počas odosielania formulára je disabled, aby sa neposlal viackrát.
                    <button
                        type="submit"
                        class="primary-button auth-submit"
                        disabled=is_pending
                    >
                        {move || {
                            // Text tlačidla sa mení podľa toho,
                            // či práve čakáme na odpoveď zo servera.
                            if is_pending.get() {
                                "Signing up..."
                            } else {
                                "Sign up"
                            }
                        }}
                    </button>
                </ActionForm>

                // Link pre používateľa, ktorý už má účet.
                <p class="auth-redirect">
                    "Already have an account? "
                    <a href="/login">"Log in"</a>
                </p>
            </div>
        </div>
    }
}