// Importujeme základné veci z Leptosu, napríklad komponenty, Resource,
// Transition a view! makro.
use leptos::prelude::*;

// Redirect použijeme na presmerovanie používateľa.
// Outlet je miesto, kde sa zobrazí chránený obsah stránky.
use leptos_router::components::{Redirect, Outlet};

// Serverová funkcia, ktorá zistí, či je používateľ aktuálne prihlásený.
use crate::server::auth::get_current_user;

// Tento komponent chráni stránky, ktoré majú byť dostupné iba pre prihlásených používateľov.
// Ak používateľ prihlásený je, pustí ho ďalej.
// Ak nie je prihlásený, presmeruje ho na login stránku.
#[component]
pub fn ProtectedRoute() -> impl IntoView {
    // Resource automaticky zavolá serverovú funkciu get_current_user.
    // Táto funkcia pravdepodobne vráti Some(user_id), ak je používateľ prihlásený,
    // alebo None, ak prihlásený nie je.
    let auth_resource = Resource::new(|| (), |_| async move { get_current_user().await });

    view! {
        // Transition zobrazí fallback počas toho, ako sa overuje prihlásenie používateľa.
        <Transition fallback=move || {
            view! { <div class="loading">"Verifying access..."</div> }
        }>
            {move || {
                // Keď Resource dokončí načítanie, získame výsledok overenia používateľa.
                auth_resource.get().map(|auth_res| match auth_res {
                    // Ak server vráti Ok(Some(_user_id)), používateľ je prihlásený.
                    // Vtedy zobrazíme Outlet, čiže reálny obsah chránenej stránky.
                    Ok(Some(_user_id)) => view! { <Outlet /> }.into_any(),

                    // V každom inom prípade používateľa presmerujeme na /login.
                    // Patrí sem napríklad:
                    // - používateľ nie je prihlásený,
                    // - server vrátil chybu,
                    // - session už neplatí.
                    _ => view! { <Redirect path="/login" /> }.into_any(),
                })
            }}
        </Transition>
    }
}