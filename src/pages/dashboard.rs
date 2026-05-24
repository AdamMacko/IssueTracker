// Importujeme základné veci z Leptosu, napríklad komponenty,
// signály, Resource, Suspense a view! makro.
use leptos::prelude::*;

// Sidebar je bočné menu aplikácie.
use crate::components::sidebar::Sidebar;

// Importujeme serverové funkcie pre projekty.
// get_projects načíta zoznam projektov.
// get_project_stats načíta štatistiky konkrétneho projektu.
use crate::server::projects::{get_project_stats, get_projects};

// Toaster používame na zobrazovanie hlášok používateľovi.
// V tomto komponente je zatiaľ iba vytiahnutý z contextu, ale priamo sa nepoužíva.
use crate::components::toast::Toaster;

// Tento komponent predstavuje dashboard stránku.
// Zobrazuje analytiku tímu, hlavne koľko času členovia odpracovali na projekte.
#[component]
pub fn DashboardPage() -> impl IntoView {
    // Z contextu si vytiahneme toaster.
    // Podčiarkovník v názve _toaster znamená, že premenná môže byť nepoužitá bez warningu.
    let _toaster = expect_context::<Toaster>();

    // Resource načíta všetky projekty zo servera.
    // Spustí sa automaticky pri otvorení dashboardu.
    let projects_res = Resource::new(
        || (),
        |_| async move { get_projects().await.unwrap_or_else(|_| vec![]) },
    );

    // selected_project_id drží ID projektu, ktorý je práve vybraný v selecte.
    // None znamená, že ešte nie je vybraný žiadny projekt.
    let (selected_project_id, set_selected_project_id) = signal::<Option<i32>>(None);

    // Resource načítava štatistiky pre aktuálne vybraný projekt.
    // Keď sa zmení selected_project_id, štatistiky sa načítajú znova.
    let stats_res = Resource::new(
        move || selected_project_id.get(),
        |proj_id| async move {
            match proj_id {
                // Ak máme vybraný projekt, zavoláme serverovú funkciu get_project_stats.
                // Pri chybe použijeme prázdny zoznam, aby UI nespadlo.
                Some(id) => get_project_stats(id).await.unwrap_or_else(|_| vec![]),

                // Ak ešte nie je vybraný projekt, štatistiky sú prázdne.
                None => vec![],
            }
        },
    );

    view! {
        <div class="dashboard-layout">
            // Bočné menu aplikácie.
            <Sidebar />

            <main class="dashboard-content">
                <header class="dashboard-header">
                    <div class="dashboard-header__top">
                        <div>
                            // Nadpis dashboardu.
                            <h1>"Team Analytics"</h1>

                            // Krátky popis stránky.
                            <p class="dashboard-header__subtitle">
                                "Overview of time distribution across the team."
                            </p>
                        </div>

                        // Suspense zobrazí fallback, kým sa načítava zoznam projektov.
                        <Suspense fallback=move || {
                            view! {
                                <div class="project-select project-select--loading">
                                    "Loading..."
                                </div>
                            }
                        }>
                            {move || {
                                // Zoberieme načítané projekty.
                                // Ak ešte nie sú načítané, použije sa prázdny zoznam.
                                let projects = projects_res.get().unwrap_or_default();

                                // Ak ešte nie je vybraný projekt a existuje aspoň jeden projekt,
                                // automaticky vyberieme prvý projekt zo zoznamu.
                                if selected_project_id.get_untracked().is_none()
                                    && !projects.is_empty()
                                {
                                    set_selected_project_id.set(Some(projects[0].id));
                                }

                                view! {
                                    // Select na výber projektu.
                                    // Po zmene projektu sa nastaví selected_project_id.
                                    <select
                                        class="project-select"
                                        on:change=move |ev| {
                                            if let Ok(id) =
                                                event_target_value(&ev).parse::<i32>()
                                            {
                                                set_selected_project_id.set(Some(id));
                                            }
                                        }
                                    >
                                        // Každý projekt zobrazíme ako jednu option v selecte.
                                        {projects
                                            .into_iter()
                                            .map(|p| {
                                                view! {
                                                    <option
                                                        value=p.id.to_string()

                                                        // selected určuje, ktorá option je aktuálne vybraná.
                                                        selected=move || {
                                                            selected_project_id.get()
                                                                == Some(p.id)
                                                        }
                                                    >
                                                        {p.name.clone()}
                                                    </option>
                                                }
                                            })
                                            .collect_view()}
                                    </select>
                                }
                            }}
                        </Suspense>
                    </div>
                </header>

                // Suspense zobrazí loading, kým sa počítajú alebo načítavajú štatistiky.
                <Suspense fallback=move || {
                    view! {
                        <div class="dashboard-loading">"Computing statistics..."</div>
                    }
                }>
                    {move || {
                        // Z Resource si zoberieme štatistiky projektu.
                        // Ak ešte nie sú načítané, použije sa prázdny zoznam.
                        let stats_data = stats_res.get().unwrap_or_default();

                        // Ak nemáme žiadne štatistiky, zobrazíme prázdny stav.
                        if stats_data.is_empty() {
                            return view! {
                                <div class="dashboard-empty">
                                    <span class="dashboard-empty__icon">"📊"</span>
                                    <p>
                                        "No time tracked yet for this project."
                                    </p>
                                </div>
                            }
                            .into_any();
                        }

                        // Spočítame celkový čas v minútach.
                        // V dátach je hodnota hours, preto ju násobíme 60.
                        let total_minutes: f32 =
                            stats_data.iter().map(|s| s.hours * 60.0).sum();

                        // Celkové minúty prevedieme späť na hodiny pre zobrazenie.
                        let total_hours: f32 = total_minutes / 60.0;

                        // Počet aktívnych členov je počet záznamov v štatistikách.
                        let active_members = stats_data.len();

                        // Priemerný počet hodín na člena.
                        // Kontrola active_members > 0 je ochrana pred delením nulou.
                        let avg_hours: f32 = if active_members > 0 {
                            total_hours / active_members as f32
                        } else {
                            0.0
                        };

                        // Nájdeme člena s najväčším počtom odpracovaných hodín.
                        let top_member = stats_data
                            .iter()
                            .max_by(|a, b| a.hours.partial_cmp(&b.hours).unwrap());

                        // Z top člena zoberieme meno.
                        // Ak by žiadny neexistoval, použije sa prázdny string.
                        let top_member_name = top_member
                            .map(|m| m.name.clone())
                            .unwrap_or_default();

                        // max_minutes používame na výpočet šírky stĺpcov v grafe.
                        // Najväčšia hodnota bude mať 100 % šírku.
                        // .max(1.0) je ochrana, aby sme nedelili nulou.
                        let max_minutes: f32 = stats_data
                            .iter()
                            .map(|s| s.hours * 60.0)
                            .fold(0.0_f32, f32::max)
                            .max(1.0);

                        view! {
                            <>
                                // Horné kartičky so základnými štatistikami.
                                <div class="stats-grid">
                                    <div class="stat-card">
                                        <span class="stat-card__label">
                                            "Total Time Tracked"
                                        </span>
                                        <span class="stat-card__value">
                                            {format!("{:.1} h", total_hours)}
                                        </span>
                                    </div>

                                    <div class="stat-card">
                                        <span class="stat-card__label">
                                            "Active Members"
                                        </span>
                                        <span class="stat-card__value">
                                            {active_members}
                                        </span>
                                    </div>

                                    <div class="stat-card">
                                        <span class="stat-card__label">
                                            "Avg. per Member"
                                        </span>
                                        <span class="stat-card__value">
                                            {format!("{:.1} h", avg_hours)}
                                        </span>
                                    </div>

                                    <div class="stat-card stat-card--highlight">
                                        <span class="stat-card__label">
                                            "Top Contributor"
                                        </span>
                                        <span class="stat-card__value stat-card__value--name">
                                            {top_member_name}
                                        </span>
                                    </div>
                                </div>

                                // Kontajner pre jednoduchý bar chart graf.
                                <div class="chart-container">
                                    <div class="chart-container__header">
                                        <h3>"Time worked per member"</h3>
                                        <span class="chart-container__legend">
                                            "minutes"
                                        </span>
                                    </div>

                                    <div class="bar-chart">
                                        // Pre každého člena tímu vytvoríme jeden riadok grafu.
                                        {stats_data
                                            .into_iter()
                                            .map(|member| {
                                                // Hodiny člena prevedieme na minúty.
                                                let minutes = member.hours * 60.0;

                                                // Vypočítame percentuálnu šírku baru.
                                                // Člen s najvyšším časom bude mať 100 %.
                                                let bar_pct = (minutes
                                                    / max_minutes
                                                    * 100.0)
                                                    .min(100.0);

                                                // Percentá prevedieme na CSS string, napríklad "75.0%".
                                                let bar_width =
                                                    format!("{:.1}%", bar_pct);

                                                // Farba baru príde zo servera / dát.
                                                let bar_color = member.color.clone();

                                                // share_pct hovorí, aký podiel z celkového času má tento člen.
                                                let share_pct = if total_minutes > 0.0
                                                {
                                                    (minutes / total_minutes * 100.0)
                                                        as u32
                                                } else {
                                                    0
                                                };

                                                view! {
                                                    // Jeden riadok v grafe pre konkrétneho člena.
                                                    <div class="chart-row">
                                                        // Meno člena tímu.
                                                        <span class="chart-row__name">
                                                            {member.name.clone()}
                                                        </span>

                                                        // Track je pozadie baru.
                                                        <div class="chart-row__track">
                                                            // Samotný farebný bar.
                                                            // Šírka a farba sa nastavujú cez inline style.
                                                            <div
                                                                class="chart-row__bar"
                                                                style=format!(
                                                                    "width: {}; background-color: {};",
                                                                    bar_width, bar_color
                                                                )
                                                            />
                                                        </div>

                                                        <div class="chart-row__meta">
                                                            // Počet odpracovaných minút.
                                                            <span class="chart-row__minutes">
                                                                {format!(
                                                                    "{:.0} min",
                                                                    minutes
                                                                )}
                                                            </span>

                                                            // Percentuálny podiel z celkového času.
                                                            <span class="chart-row__share">
                                                                {format!(
                                                                    "{}%",
                                                                    share_pct
                                                                )}
                                                            </span>
                                                        </div>
                                                    </div>
                                                }
                                            })
                                            .collect_view()}
                                    </div>
                                </div>
                            </>
                        }
                        .into_any()
                    }}
                </Suspense>
            </main>
        </div>
    }
}