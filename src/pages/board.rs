// Importujeme základné veci z Leptosu, napríklad komponenty,
// signály, Resource, Transition a view! makro.
use leptos::prelude::*;

// Hooky z routera.
// use_params_map použijeme na získanie ID projektu z URL.
// use_navigate použijeme na presmerovanie používateľa.
use leptos_router::hooks::{use_params_map, use_navigate};

// spawn_local používame na spustenie async kódu na klientovi,
// napríklad pri presúvaní tasku alebo priraďovaní používateľa.
use leptos::task::spawn_local;

// Sidebar je bočné menu aplikácie.
use crate::components::sidebar::Sidebar;

// Importujeme serverové funkcie a DTO pre tasky.
// get_tasks načíta tasky projektu.
// update_task_status zmení status tasku.
// assign_task priradí task používateľovi.
// TaskDto je dátový typ tasku, ktorý príde zo servera.
use crate::server::tasks::{get_tasks, update_task_status, assign_task, TaskDto};

// Serverová funkcia na načítanie členov projektu.
use crate::server::projects::get_project_members;

// Modal na vytvorenie nového tasku.
use crate::components::new_task_modal::NewTaskModal;

// Modal s detailom konkrétneho tasku.
use crate::components::task_detail_modal::TaskDetailModal;

// Toaster používame na zobrazovanie úspešných alebo chybových hlášok.
use crate::components::toast::Toaster;

// Tento komponent predstavuje Kanban board pre konkrétny projekt.
// Zobrazuje tasky v stĺpcoch podľa statusu a umožňuje ich presúvať.
#[component]
pub fn BoardPage() -> impl IntoView {
    // Z contextu si vytiahneme toaster na notifikácie.
    let toaster = expect_context::<Toaster>();

    // Získame parametre z URL.
    // Napríklad ak máme URL /projects/5/board, tak odtiaľ vieme získať id projektu.
    let params = use_params_map();

    // Hook na presmerovanie používateľa na inú stránku.
    let navigate = use_navigate();

    // project_id_str vracia ID projektu ako String.
    // Používa sa hlavne na zobrazenie v breadcrumbs.
    let project_id_str = move || {
        params.with(|p| p.get("id").unwrap_or_default())
    };

    // project_id_num vracia ID projektu ako číslo i64.
    // Toto ID sa používa pri volaní serverových funkcií.
    // Ak sa ID nepodarí načítať alebo parsovať, použije sa 0.
    let project_id_num = move || {
        params.with(|p| p.get("id").and_then(|id| id.parse::<i64>().ok()).unwrap_or(0))
    };

    // selected_task_id drží ID tasku, ktorý je práve otvorený v detaile.
    // None znamená, že detail žiadneho tasku nie je otvorený.
    let (selected_task_id, set_selected_task_id) = signal::<Option<i64>>(None);

    // Signál, ktorý hovorí, či je modal na vytvorenie tasku otvorený.
    let is_task_modal_open = RwSignal::new(false);

    // reload_trigger používame na znovunačítanie taskov.
    // Keď sa jeho hodnota zmení, Resource s taskami sa spustí znova.
    let (reload_trigger, set_reload_trigger) = signal(0);

    // dragged_task_id drží ID tasku, ktorý používateľ práve ťahá drag-and-dropom.
    // None znamená, že sa práve nič nepresúva.
    let dragged_task_id = RwSignal::new(None::<i64>);

    // Callback, ktorý sa zavolá po úspešnom vytvorení tasku.
    // Zvýšením reload_trigger vynútime refresh boardu.
    let on_task_created = Callback::new(move |_| {
        set_reload_trigger.update(|n| *n += 1);
    });

    // Resource načítava všetky tasky pre aktuálny projekt.
    // Závisí od project_id_num a reload_trigger.
    // Čiže keď sa zmení projekt alebo reload_trigger, tasky sa načítajú znovu.
    let tasks_resource = Resource::new(
        move || (project_id_num(), reload_trigger.get()),
        |(id, _)| async move {
            // Ak je ID projektu 0, znamená to neplatné ID.
            if id == 0 {
                return Err(ServerFnError::new("Invalid project ID"));
            }

            // Zavoláme serverovú funkciu, ktorá načíta tasky projektu.
            get_tasks(id).await
        },
    );

    // Resource načítava členov aktuálneho projektu.
    // Používa sa hlavne v selecte na priradenie tasku používateľovi.
    let members_resource = Resource::new(
        move || project_id_num(),
        |id| async move {
            // Ak je ID projektu neplatné, vrátime prázdny zoznam.
            if id == 0 {
                return vec![];
            }

            // Načítame členov projektu zo servera.
            // Ak nastane chyba, použije sa prázdny zoznam.
            get_project_members(id).await.unwrap_or_default()
        },
    );

    // Effect sleduje, či sa tasky podarilo načítať.
    // Ak server vráti chybu, používateľa presmerujeme na dashboard.
    Effect::new(move |_| {
        if let Some(Err(_)) = tasks_resource.get() {
            navigate("/dashboard", Default::default());
        }
    });

    view! {
        <div class="dashboard-layout">
            // Bočné menu aplikácie.
            <Sidebar />

            <main class="dashboard-content">
                <header class="dashboard-header">
                    <div style="display: flex; justify-content: space-between; align-items: flex-start;">
                        <div>
                            // Breadcrumbs ukazujú, kde sa používateľ nachádza.
                            <div class="breadcrumbs">
                                <a href="/projects">"Projects"</a>
                                <span class="separator">"/"</span>
                                <span>"Project #" {project_id_str}</span>
                            </div>

                            // Nadpis stránky.
                            <h1 style="margin-top: 0.25rem;">"Kanban Board"</h1>
                        </div>

                        <div style="display: flex; gap: 1rem;">
                            // Tlačidlo na otvorenie modalu pre vytvorenie nového tasku.
                            <button
                                class="primary-button"
                                style="height: 36px;"
                                on:click=move |_| is_task_modal_open.set(true)
                            >
                                "+ Task"
                            </button>
                        </div>
                    </div>
                </header>

                <section class="kanban-wrapper">
                    // Transition zobrazí loading text, kým sa načítavajú tasky.
                    <Transition fallback=move || {
                        view! { <div class="loading">"Loading board..."</div> }
                    }>
                        {move || {
                            // Z Resource si zoberieme tasky.
                            // Ak ešte nie sú načítané alebo nastala chyba, použije sa prázdny zoznam.
                            let all_tasks = tasks_resource
                                .get()
                                .and_then(|res| res.ok())
                                .unwrap_or_default();

                            view! {
                                <div class="kanban-board">
                                    // Vytvoríme 4 Kanban stĺpce.
                                    // Každý stĺpec reprezentuje jeden status tasku.
                                    {["To Do", "In Progress", "In Review", "Done"]
                                        .into_iter()
                                        .map(|col_name| {
                                            // Text v UI je pekne čitateľný, ale v databáze/serveri
                                            // používame trochu iné hodnoty statusu.
                                            let db_status = match col_name {
                                                "To Do" => "Todo",
                                                "In Progress" => "InProgress",
                                                "In Review" => "InReview",
                                                _ => "Done",
                                            };

                                            // Vyfiltrujeme iba tasky, ktoré patria do aktuálneho stĺpca.
                                            let col_tasks: Vec<TaskDto> = all_tasks
                                                .iter()
                                                .filter(|t| t.status == db_status)
                                                .cloned()
                                                .collect();

                                            // Počet taskov v stĺpci.
                                            let count = col_tasks.len();

                                            // Status, ktorý sa nastaví tasku pri dropnutí do tohto stĺpca.
                                            let target_status = db_status.to_string();

                                            view! {
                                                // Jeden Kanban stĺpec.
                                                // Podporuje dragover a drop, aby sa doň dali presúvať tasky.
                                                <div
                                                    class="kanban-column"

                                                    // prevent_default je potrebné, aby browser dovolil drop.
                                                    on:dragover=move |ev| ev.prevent_default()

                                                    // Toto sa spustí, keď používateľ pustí task do stĺpca.
                                                    on:drop=move |ev| {
                                                        ev.prevent_default();

                                                        // Skontrolujeme, či máme uložené ID ťahaného tasku.
                                                        if let Some(task_id) = dragged_task_id.get() {
                                                            let new_status = target_status.clone();
                                                            let toaster = toaster.clone();

                                                            // Zmena statusu je async serverová operácia,
                                                            // preto ju spúšťame cez spawn_local.
                                                            spawn_local(async move {
                                                                match update_task_status(
                                                                    task_id,
                                                                    new_status.clone(),
                                                                )
                                                                .await
                                                                {
                                                                    Ok(_) => {
                                                                        // Ak sa status zmenil úspešne,
                                                                        // zobrazíme hlášku.
                                                                        toaster.success(
                                                                            format!(
                                                                                "Task moved to {}",
                                                                                new_status
                                                                            ),
                                                                        );

                                                                        // Refreshneme board, aby sa task zobrazil v novom stĺpci.
                                                                        set_reload_trigger
                                                                            .update(|n| *n += 1);
                                                                    }
                                                                    Err(e) => {
                                                                        // Chybu zo servera upravíme na čitateľnejší text.
                                                                        let err_msg = e
                                                                            .to_string()
                                                                            .replace(
                                                                                "error running server function:",
                                                                                "",
                                                                            )
                                                                            .trim()
                                                                            .to_string();

                                                                        // Zobrazíme chybovú hlášku.
                                                                        toaster.error(format!(
                                                                            "Failed to move task: {}",
                                                                            err_msg
                                                                        ));
                                                                    }
                                                                }
                                                            });
                                                        }

                                                        // Po dokončení dropu vyčistíme ID ťahaného tasku.
                                                        dragged_task_id.set(None);
                                                    }
                                                >
                                                    <div class="column-header">
                                                        <h3>
                                                            // Názov stĺpca.
                                                            {col_name}

                                                            // Počet taskov v danom stĺpci.
                                                            <span class="task-count">{count}</span>
                                                        </h3>
                                                    </div>

                                                    <div class="column-cards">
                                                        // Prejdeme všetky tasky v aktuálnom stĺpci
                                                        // a zobrazíme ich ako karty.
                                                        {col_tasks
                                                            .into_iter()
                                                            .map(|task| {
                                                                // ID tasku si uložíme zvlášť,
                                                                // aby sme ho mohli používať v event handleroch.
                                                                let task_id = task.id;

                                                                // Aktuálne priradený používateľ.
                                                                // None znamená, že task ešte nikomu nie je priradený.
                                                                let current_assignee =
                                                                    task.assignee_id;

                                                                view! {
                                                                    // Jedna karta tasku v Kanban boarde.
                                                                    <div
                                                                        class="kanban-card"

                                                                        // Povolenie drag-and-drop pre túto kartu.
                                                                        draggable="true"

                                                                        // Keď používateľ začne ťahať kartu,
                                                                        // uložíme si ID tasku.
                                                                        on:dragstart=move |_| {
                                                                            dragged_task_id.set(Some(task_id))
                                                                        }

                                                                        // Po kliknutí na kartu otvoríme detail tasku.
                                                                        on:click=move |_| {
                                                                            set_selected_task_id.set(Some(task_id))
                                                                        }
                                                                    >
                                                                        // Názov tasku.
                                                                        <p class="card-title">
                                                                            {task.title.clone()}
                                                                        </p>

                                                                        <div class="card-footer">
                                                                            // ID tasku zobrazené na karte.
                                                                            <span class="card-id">
                                                                                "#" {task.id}
                                                                            </span>

                                                                            // Select na priradenie tasku používateľovi.
                                                                            <select
                                                                                class="assignee-select"

                                                                                // Zastavíme propagáciu kliknutia,
                                                                                // aby kliknutie na select neotvorilo detail tasku.
                                                                                on:click=move |ev| {
                                                                                    ev.stop_propagation()
                                                                                }

                                                                                // Pri zmene selectu zavoláme server a priradíme task.
                                                                                on:change=move |ev| {
                                                                                    let val =
                                                                                        event_target_value(&ev);

                                                                                    // Ak používateľ vyberie "none",
                                                                                    // task nebude priradený nikomu.
                                                                                    let new_id = if val
                                                                                        == "none"
                                                                                    {
                                                                                        None
                                                                                    } else {
                                                                                        // Inak sa pokúsime hodnotu premeniť na ID používateľa.
                                                                                        val.parse::<i64>()
                                                                                            .ok()
                                                                                    };

                                                                                    let toaster = toaster
                                                                                        .clone();

                                                                                    // Priradenie tasku je async operácia,
                                                                                    // preto ju spúšťame cez spawn_local.
                                                                                    spawn_local(
                                                                                        async move {
                                                                                            match assign_task(
                                                                                                task_id, new_id,
                                                                                            )
                                                                                            .await
                                                                                            {
                                                                                                Ok(_) => {
                                                                                                    // Ak sa task úspešne priradil,
                                                                                                    // zobrazíme hlášku.
                                                                                                    toaster
                                                                                                        .success(
                                                                                                            "Task assigned",
                                                                                                        );

                                                                                                    // Refreshneme tasky, aby UI ukázalo aktuálny stav.
                                                                                                    set_reload_trigger
                                                                                                        .update(
                                                                                                            |n| {
                                                                                                                *n += 1
                                                                                                            },
                                                                                                        );
                                                                                                }
                                                                                                Err(e) => {
                                                                                                    // Chybu zo servera upravíme na čitateľnejší text.
                                                                                                    let err_msg = e
                                                                                                        .to_string()
                                                                                                        .replace(
                                                                                                            "error running server function:",
                                                                                                            "",
                                                                                                        )
                                                                                                        .trim()
                                                                                                        .to_string();

                                                                                                    // Zobrazíme chybu používateľovi.
                                                                                                    toaster
                                                                                                        .error(
                                                                                                            format!(
                                                                                                                "Failed to assign: {}",
                                                                                                                err_msg
                                                                                                            ),
                                                                                                        );
                                                                                                }
                                                                                            }
                                                                                        },
                                                                                    );
                                                                                }
                                                                            >
                                                                                // Možnosť, že task nie je nikomu priradený.
                                                                                <option
                                                                                    value="none"
                                                                                    selected=current_assignee
                                                                                        .is_none()
                                                                                >
                                                                                    "Unassigned"
                                                                                </option>

                                                                                {move || {
                                                                                    // Načítame členov projektu.
                                                                                    // Ak ešte nie sú načítaní, použije sa prázdny zoznam.
                                                                                    let members =
                                                                                        members_resource
                                                                                            .get()
                                                                                            .unwrap_or_default();

                                                                                    // Každého člena projektu zobrazíme ako možnosť v selecte.
                                                                                    members
                                                                                        .into_iter()
                                                                                        .map(|m| {
                                                                                            view! {
                                                                                                <option
                                                                                                    value=m.id

                                                                                                    // Ak je tento člen aktuálne priradený,
                                                                                                    // nastavíme option ako selected.
                                                                                                    selected=Some(
                                                                                                        m.id,
                                                                                                    ) == current_assignee
                                                                                                >
                                                                                                    {m.username
                                                                                                        .clone()}
                                                                                                </option>
                                                                                            }
                                                                                        })
                                                                                        .collect_view()
                                                                                }}
                                                                            </select>
                                                                        </div>
                                                                    </div>
                                                                }
                                                            })
                                                            .collect_view()}
                                                    </div>
                                                </div>
                                            }
                                        })
                                        .collect_view()}
                                </div>
                            }
                        }}
                    </Transition>
                </section>
            </main>

            // Modal na vytvorenie nového tasku.
            // Dostáva ID projektu a callback, ktorý po úspechu refreshne board.
            <NewTaskModal
                is_open=is_task_modal_open
                project_id=project_id_num
                on_success=on_task_created
            />

            // Modal s detailom tasku.
            // Otvorí sa podľa selected_task_id.
            <TaskDetailModal
                task_id=selected_task_id
                set_task_id=set_selected_task_id
            />
        </div>
    }
}