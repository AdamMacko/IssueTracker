// Importujeme základné veci z Leptosu, napríklad komponenty,
// signály, Resource, Effect, Transition a view! makro.
use leptos::prelude::*;

// spawn_local použijeme na spustenie async kódu na klientovi.
// Tu ho používame pri ukladaní času na server.
use leptos::task::spawn_local;

// Importujeme serverové funkcie a DTO pre time tracking.
// get_time_entries načíta uložené časové záznamy.
// add_time_entry uloží nový časový záznam.
// TimeEntryDto je dátový typ jedného časového záznamu zo servera.
use crate::server::tasks::{get_time_entries, add_time_entry, TimeEntryDto};

// Duration používame pri intervale, aby sa timer zvyšoval každú sekundu.
use std::time::Duration;

// Toaster používame na zobrazovanie hlášok používateľovi.
use crate::components::toast::Toaster;

// Tento komponent slúži na sledovanie času pri konkrétnom tasku.
// Používateľ môže spustiť timer, zastaviť ho a čas sa potom uloží na server.
#[component]
pub fn TimeTracker(
    // ID tasku, ku ktorému sa má čas zapisovať.
    // Signal používame preto, lebo task_id môže prísť dynamicky z rodičovského komponentu.
    #[prop(into)] task_id: Signal<i64>,
) -> impl IntoView {
    // Z contextu si vytiahneme toaster, aby sme vedeli zobrazovať notifikácie.
    let toaster = expect_context::<Toaster>();

    // is_running hovorí, či timer práve beží.
    // false znamená, že timer je zastavený.
    let (is_running, set_is_running) = signal(false);

    // seconds drží počet sekúnd, ktoré aktuálne odmeriava timer.
    let (seconds, set_seconds) = signal(0i64);

    // reload_trigger používame na znovunačítanie časových záznamov.
    // Keď sa jeho hodnota zmení, Resource nižšie sa spustí znova.
    let (reload_trigger, set_reload_trigger) = signal(0);

    // Resource načítava časové záznamy zo servera.
    // Závisí od task_id a reload_trigger, takže sa obnoví pri zmene tasku
    // alebo po uložení nového času.
    let time_resource = Resource::new(
        move || (task_id.get(), reload_trigger.get()),
        |(id, _)| async move {
            // Ak je id 0, nič nenačítavame.
            // Je to ochrana pred neplatným alebo ešte nenačítaným task_id.
            if id == 0 {
                return vec![];
            }

            // Načítame časové záznamy zo servera.
            // Ak nastane chyba, použije sa prázdny zoznam.
            get_time_entries(id).await.unwrap_or_default()
        },
    );

    // Effect sleduje, či timer beží.
    // Keď sa is_running nastaví na true, vytvorí sa interval,
    // ktorý každú sekundu zvýši počet sekúnd.
    Effect::new(move |_| {
        if is_running.get() {
            // Nastavíme interval, ktorý sa spustí každú sekundu.
            let handle = set_interval_with_handle(
                move || {
                    // Každú sekundu zvýšime seconds o 1.
                    set_seconds.update(|s| *s += 1);
                },
                Duration::from_secs(1),
            )
            .expect("Failed to set interval");

            // Keď sa Effect zruší alebo znovu spustí, interval vyčistíme.
            // Toto je dôležité, aby nám nebežalo viac timerov naraz.
            on_cleanup(move || {
                handle.clear();
            });
        }
    });

    // Funkcia, ktorá sa spustí po kliknutí na Stop.
    let stop_timer = move |_| {
        // Uložíme si aktuálny počet odmeraných sekúnd.
        let logged_secs = seconds.get();

        // Timer zastavíme.
        set_is_running.set(false);

        // Čas ukladáme iba vtedy, ak je väčší ako 0 sekúnd.
        if logged_secs > 0 {
            // Zoberieme ID aktuálneho tasku.
            let tid = task_id.get();

            // Toaster si naklonujeme, aby sa dal použiť v async bloku.
            let toaster = toaster.clone();

            // Serverová funkcia je async, preto ju spúšťame cez spawn_local.
            spawn_local(async move {
                match add_time_entry(tid, logged_secs).await {
                    Ok(_) => {
                        // Ak sa čas úspešne uloží, zobrazíme hlášku.
                        toaster.success("Time entry saved");

                        // Vynulujeme timer.
                        set_seconds.set(0);

                        // Vynútime refresh časových záznamov.
                        set_reload_trigger.update(|n| *n += 1);
                    }
                    Err(e) => {
                        // Chybovú správu zo servera si upravíme na čitateľnejší text.
                        let err_msg = e
                            .to_string()
                            .replace("error running server function:", "")
                            .trim()
                            .to_string();

                        // Zobrazíme chybu používateľovi.
                        toaster.error(format!("Failed to save time: {}", err_msg));
                    }
                }
            });
        }
    };

    // Táto funkcia formátuje aktuálny čas timeru.
    // Zo sekúnd spraví text vo formáte HH:MM:SS.
    let formatted_time = move || {
        let s = seconds.get();

        // Hodiny vypočítame delením počtu sekúnd číslom 3600.
        let h = s / 3600;

        // Minúty vypočítame zo zvyšku po hodinách.
        let m = (s % 3600) / 60;

        // Sekundy sú zvyšok po delení 60.
        let sec = s % 60;

        // Výsledok bude napríklad 00:01:25.
        format!("{:02}:{:02}:{:02}", h, m, sec)
    };

    view! {
        <div class="time-tracker-box">
            <div class="tracker-label">
                "Time Tracking "

                // Transition sa používa pri načítavaní celkového odpracovaného času.
                <Transition fallback=|| ()>
                    {move || {
                        // Z Resource si vytiahneme časové záznamy.
                        // Ak ešte nie sú načítané, použije sa prázdny zoznam.
                        let entries = time_resource.get().unwrap_or_default();

                        // Spočítame všetky uložené sekundy zo všetkých time entry záznamov.
                        let total_secs: i64 =
                            entries.iter().map(|e: &TimeEntryDto| e.duration_seconds).sum();

                        if total_secs > 0 {
                            // Celkový čas prevedieme na hodiny a minúty.
                            let h = total_secs / 3600;
                            let m = (total_secs % 3600) / 60;

                            view! {
                                // Vpravo hore zobrazíme celkový odpracovaný čas.
                                <span style="color: #64748b; font-size: 0.8rem; float: right;">
                                    "Total: " {h} "h " {m} "m"
                                </span>
                            }
                            .into_any()
                        } else {
                            // Ak ešte nie je nič odpracované, nezobrazíme nič špeciálne.
                            view! { <span></span> }.into_any()
                        }
                    }}
                </Transition>
            </div>

            <div class="tracker-main">
                // Hlavné zobrazenie bežiaceho timeru.
                // class:active sa pridá, keď timer práve beží.
                <div class="timer-display" class:active=move || is_running.get()>
                    {formatted_time}
                </div>

                <div class="tracker-controls">
                    // Ak timer nebeží, zobrazíme Start tlačidlo.
                    // Ak timer beží, fallback zobrazí Stop tlačidlo.
                    <Show
                        when=move || !is_running.get()
                        fallback=move || {
                            view! {
                                // Stop tlačidlo zastaví timer a uloží čas na server.
                                <button class="stop-btn" on:click=stop_timer>
                                    "■ Stop"
                                </button>
                            }
                        }
                    >
                        // Start tlačidlo spustí timer.
                        <button
                            class="start-btn"
                            on:click=move |_| set_is_running.set(true)
                        >
                            "▶ Start"
                        </button>
                    </Show>
                </div>
            </div>

            <div class="time-logs">
                // Transition zobrazí Loading, kým sa načítavajú uložené časové záznamy.
                <Transition fallback=|| {
                    view! {
                        <div style="font-size: 0.85rem; color: #94a3b8;">
                            "Loading..."
                        </div>
                    }
                }>
                    {move || {
                        // Zoberieme uložené time entries.
                        let entries = time_resource.get().unwrap_or_default();

                        if entries.is_empty() {
                            // Ak ešte nie je uložený žiadny čas, zobrazíme jednoduchú správu.
                            view! {
                                <div style="color: #94a3b8; font-size: 0.85rem; text-align: center;">
                                    "No time logged yet."
                                </div>
                            }
                            .into_any()
                        } else {
                            // Ak existujú časové záznamy, vypíšeme ich pod timer.
                            entries
                                .into_iter()
                                .map(|entry| {
                                    // Duration_seconds je uložený čas v sekundách.
                                    let s = entry.duration_seconds;

                                    // Prevedieme sekundy na hodiny a minúty.
                                    let h = s / 3600;
                                    let m = (s % 3600) / 60;

                                    // Ak je čas dlhší ako hodina, zobrazíme aj hodiny.
                                    // Inak zobrazíme iba minúty.
                                    let duration_str = if h > 0 {
                                        format!("{}h {}m", h, m)
                                    } else {
                                        format!("{}m", m)
                                    };

                                    view! {
                                        // Jeden riadok časového záznamu.
                                        <div class="log-item">
                                            // Meno používateľa, ktorý čas zapísal.
                                            <span style="font-size: 0.85rem; color: #475569;">
                                                <strong>{entry.user_name.clone()}</strong>
                                            </span>

                                            // Dĺžka odpracovaného času.
                                            <span class="log-duration">{duration_str}</span>
                                        </div>
                                    }
                                })
                                .collect_view()
                                .into_any()
                        }
                    }}
                </Transition>
            </div>
        </div>
    }
}