// Importujeme základné veci z Leptosu, napríklad komponenty,
// signály, Resource, Show, Suspense a view! makro.
use leptos::prelude::*;

// Import komponentu pre diskusiu ku konkrétnemu tasku.
use crate::components::comments::IssueDiscussion;

// Import komponentu na meranie času pri taske.
use crate::components::time_tracker::TimeTracker;

// Serverová funkcia, ktorá načíta detail konkrétneho tasku podľa jeho ID.
use crate::server::tasks::get_task;

// Toaster používame na zobrazovanie hlášok používateľovi.
use crate::components::toast::Toaster;

// Tento komponent predstavuje modal s detailom tasku.
// Zobrazí názov tasku, status, prioritu, popis, time tracker a komentáre.
#[component]
pub fn TaskDetailModal(
    // task_id drží ID tasku, ktorý je práve otvorený v modale.
    // Option<i64> znamená, že tam buď je Some(id), alebo None.
    // None znamená, že modal nie je otvorený.
    task_id: ReadSignal<Option<i64>>,

    // set_task_id používame na zmenu task_id.
    // Keď nastavíme None, modal sa zavrie.
    set_task_id: WriteSignal<Option<i64>>,
) -> impl IntoView {
    // Z contextu si vytiahneme toaster na zobrazovanie notifikácií.
    let toaster = expect_context::<Toaster>();

    // Funkcia na zatvorenie modalu.
    // Nastaví task_id na None, takže Show nižšie prestane modal zobrazovať.
    let close_modal = move |_| {
        set_task_id.set(None);
    };

    // Resource načítava detail tasku zo servera.
    // Spustí sa vždy, keď sa zmení task_id.
    let task_res = Resource::new(
        // Zdrojová hodnota pre Resource je aktuálne task_id.
        move || task_id.get(),

        // Podľa task_id sa rozhodneme, či máme načítať task zo servera.
        |id_opt| async move {
            match id_opt {
                // Ak máme ID tasku, zavoláme serverovú funkciu get_task.
                // .ok() zmení Result na Option, čiže pri chybe dostaneme None.
                Some(id) => get_task(id).await.ok(),

                // Ak task_id nie je nastavené, nič nenačítavame.
                None => None,
            }
        },
    );

    // Funkcia, ktorá sa spustí po kliknutí na Save Description.
    // Momentálne iba zobrazí hlášku, reálne ukladanie na server tu ešte nie je.
    let handle_save_description = move |_| {
        toaster.success("Description saved successfully");
    };

    view! {
        // Modal zobrazíme iba vtedy, keď existuje nejaké task_id.
        // Čiže keď task_id je Some(id).
        <Show when=move || task_id.get().is_some()>
            // Pozadie za modalom.
            // Kliknutie na toto pozadie zavrie modal.
            <div class="modal-backdrop" on:click=close_modal></div>

            // Kontajner celého modalu.
            <div class="task-modal-container">
                <div class="task-modal">
                    <div class="modal-header">
                        <div class="header-left">
                            // Badge s ID tasku, napríklad TASK-5.
                            // unwrap_or(0) je poistka, keby task_id náhodou nebolo nastavené.
                            <span class="task-id-badge">
                                "TASK-" {move || task_id.get().unwrap_or(0)}
                            </span>
                        </div>

                        // Tlačidlo na zatvorenie modalu.
                        <button class="close-btn" on:click=close_modal>
                            "✕"
                        </button>
                    </div>

                    <div class="modal-content">
                        // Suspense zobrazí fallback počas toho, ako sa načítava detail tasku.
                        <Suspense fallback=move || {
                            view! { <div class="loading">"Loading details..."</div> }
                        }>
                            {move || {
                                // Z Resource si zoberieme načítaný task.
                                // task_res.get() vráti Option<Option<Task>>,
                                // preto je tu flatten(), aby sme z toho spravili jednoduchšie Option<Task>.
                                task_res.get().flatten().map(|task| {
                                    // Názov a popis si uložíme do premenných.
                                    // clone() je tu preto, aby sme si hodnoty vedeli bezpečne použiť vo view.
                                    let title = task.title.clone();

                                    // Popis môže byť None, preto použijeme unwrap_or_default().
                                    // Ak popis neexistuje, použije sa prázdny String.
                                    let desc = task.description.clone().unwrap_or_default();

                                    // Toto je dôležitá oprava.
                                    // ID si zoberieme priamo z načítaného tasku a potom ho posielame ďalej.
                                    // Vďaka tomu TimeTracker a IssueDiscussion nemusia pri zatváraní modalu
                                    // stále čítať parent signál task_id.
                                    let loaded_task_id = task.id;

                                    view! {
                                        // Layout detailu tasku.
                                        // Vľavo sú hlavné informácie, vpravo sidebar.
                                        <div class="task-layout">
                                            <div class="task-main">
                                                // Nadpis tasku.
                                                <h1 class="task-title">{title}</h1>

                                                // Bar so základnými meta informáciami o taske.
                                                <div class="task-meta-bar">
                                                    <div class="meta-item">
                                                        <label>"Status"</label>

                                                        // Zobrazujeme aktuálny status tasku zo servera.
                                                        <span class="status-badge">
                                                            {task.status.clone()}
                                                        </span>
                                                    </div>

                                                    <div class="meta-item">
                                                        <label>"Priority"</label>

                                                        // Select pre prioritu tasku.
                                                        // Momentálne tu je natvrdo označená Medium priorita.
                                                        <select class="priority-select">
                                                            <option value="low">"Low"</option>
                                                            <option value="medium" selected=true>
                                                                "Medium"
                                                            </option>
                                                            <option value="high">"High"</option>
                                                            <option value="urgent">"Urgent"</option>
                                                        </select>
                                                    </div>
                                                </div>

                                                // Sekcia s popisom tasku.
                                                <div class="task-description">
                                                    <h3>"Description"</h3>

                                                    // Textarea obsahuje aktuálny popis tasku.
                                                    // Používateľ ho môže upraviť, ale reálne uloženie zatiaľ nie je napojené.
                                                    <textarea
                                                        class="desc-input"
                                                        placeholder="Add task details here..."
                                                    >
                                                        {desc}
                                                    </textarea>

                                                    <div class="desc-actions">
                                                        // Tlačidlo na uloženie popisu.
                                                        // Aktuálne iba zobrazí toaster hlášku.
                                                        <button
                                                            class="primary-button small"
                                                            on:click=handle_save_description
                                                        >
                                                            "Save Description"
                                                        </button>
                                                    </div>
                                                </div>
                                            </div>

                                            // Pravá časť detailu tasku.
                                            // Sú tu doplnkové komponenty ako time tracker a diskusia.
                                            <div class="task-sidebar">
                                                <div class="sidebar-widget">
                                                    // Komponent na sledovanie času pre aktuálny task.
                                                    // Teraz dostáva stabilné loaded_task_id.
                                                    <TimeTracker task_id=move || loaded_task_id />
                                                </div>

                                                <div class="sidebar-widget">
                                                    // Komponent s komentármi / diskusiou k aktuálnemu tasku.
                                                    // Tiež dostáva stabilné loaded_task_id.
                                                    <IssueDiscussion task_id=move || loaded_task_id />
                                                </div>
                                            </div>
                                        </div>
                                    }
                                }).into_any()
                            }}
                        </Suspense>
                    </div>
                </div>
            </div>
        </Show>
    }
}