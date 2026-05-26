// Importujeme základné veci z Leptosu, napríklad komponenty,
// signály, Resource, Transition a view! makro.
use leptos::prelude::*;

// Komponent A je link z leptos_router.
// Používa sa namiesto klasického <a>, aby navigácia fungovala cez router.
use leptos_router::components::A;

// Sidebar je bočné menu aplikácie.
use crate::components::sidebar::Sidebar;

// Modal na vytvorenie nového projektu.
use crate::components::new_project_modal::NewProjectModal;

// Importujeme serverovú funkciu a dátový typ pre projekty.
// get_projects načíta projekty zo servera.
// Project je struct jedného projektu.
use crate::server::projects::{get_projects, Project};

// Toaster používame na zobrazovanie hlášok používateľovi.
// Tu ho síce vytiahneme z contextu, ale priamo ho nepoužívame.
use crate::components::toast::Toaster;

// Toto je stránka so zoznamom projektov.
// Zobrazuje všetky projekty a umožňuje otvoriť modal na vytvorenie nového projektu.
#[component]
pub fn ProjectsPage() -> impl IntoView {
    // Z contextu si vytiahneme toaster.
    // Podčiarkovník znamená, že premenná môže zostať nepoužitá bez warningu.
    let _toaster = expect_context::<Toaster>();

    // Signál, ktorý určuje, či je modal na vytvorenie projektu otvorený.
    let is_modal_open = RwSignal::new(false);

    // reload_trigger používame na refresh zoznamu projektov.
    // Keď sa jeho hodnota zmení, Resource sa spustí znova.
    let (reload_trigger, set_reload_trigger) = signal(0);

    // Resource načítava projekty zo servera.
    // Závisí od reload_trigger, takže sa obnoví po vytvorení nového projektu.
    let projects_resource = Resource::new(
        move || reload_trigger.get(),
        |_| async move { get_projects().await },
    );

    // Callback, ktorý sa zavolá po úspešnom vytvorení projektu.
    // Zvýšením reload_trigger vynútime znovunačítanie projektov.
    let on_project_created = Callback::new(move |_| {
        set_reload_trigger.update(|n| *n += 1);
    });

    view! {
        <div class="dashboard-layout">
            // Bočné menu aplikácie.
            <Sidebar />

            <main class="dashboard-content">
                <header class="dashboard-header">
                    <div style="display: flex; justify-content: space-between; align-items: center;">
                        <div>
                            // Nadpis stránky.
                            <h1>"Projects"</h1>

                            // Krátky popis stránky.
                            <p>"Manage all your personal and shared projects."</p>
                        </div>

                        // Tlačidlo otvorí modal na vytvorenie nového projektu.
                        <button
                            class="primary-button"
                            style="min-width: auto; height: 40px;"
                            on:click=move |_| is_modal_open.set(true)
                        >
                            "+ New Project"
                        </button>
                    </div>
                </header>

                <section class="projects-section">
                    // Transition zobrazí fallback, kým sa načítavajú projekty.
                    <Transition fallback=move || {
                        view! { <div class="loading">"Loading projects..."</div> }
                    }>
                        {move || {
                            // Zoberieme výsledok z Resource.
                            projects_resource.get().map(|res| {
                                match res {
                                    Ok(data) if data.is_empty() => {
                                        // Ak server vráti prázdny zoznam, zobrazíme empty state.
                                        view! {
                                            <div class="empty-state">
                                                "No projects found."
                                            </div>
                                        }
                                        .into_any()
                                    }
                                    Ok(data) => {
                                        // Ak máme projekty, zobrazíme ich v gride ako ProjectCard.
                                        view! {
                                            <div class="projects-grid">
                                                {data
                                                    .into_iter()
                                                    .map(|project| {
                                                        view! {
                                                            <ProjectCard project />
                                                        }
                                                    })
                                                    .collect_view()}
                                            </div>
                                        }
                                        .into_any()
                                    }
                                    Err(_) => {
                                        // Ak nastane chyba pri načítaní projektov,
                                        // zobrazíme jednoduchú chybovú hlášku.
                                        view! {
                                            <div class="error">
                                                "Failed to load projects."
                                            </div>
                                        }
                                        .into_any()
                                    }
                                }
                            })
                        }}
                    </Transition>
                </section>
            </main>

            // Modal na vytvorenie nového projektu.
            // Po úspechu sa zavolá on_project_created, ktorý refreshne zoznam.
            <NewProjectModal is_open=is_modal_open on_success=on_project_created />
        </div>
    }
}

// Komponent pre jednu kartu projektu.
// Každý projekt v zozname sa zobrazí pomocou tejto karty.
#[component]
fn ProjectCard(project: Project) -> impl IntoView {
    // Progress je momentálne natvrdo 0.
    // Neskôr by sa sem mohol doplniť reálny výpočet dokončenia projektu.
    let progress = 0;

    // Role je momentálne natvrdo Owner.
    // Neskôr by sa mohla načítať skutočná rola používateľa v projekte.
    let role = "Owner";

    // Vytvoríme URL na board konkrétneho projektu.
    // Napríklad pre projekt s ID 5 vznikne /projects/5/board.
    let board_url = format!("/projects/{}/board", project.id);

    view! {
        // Celá karta je klikateľný link na Kanban board projektu.
        <A href=board_url attr:class="project-card">
            <div class="project-card-header">
                // Názov projektu.
                <h3>{project.name.clone()}</h3>

                // Badge s rolou používateľa v projekte.
                <span class="role-badge owner">{role}</span>
            </div>

            // Popis projektu.
            <p class="project-desc">{project.description.clone()}</p>

            
        </A>
    }
}