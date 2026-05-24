// Importujeme základné veci z Leptosu, napríklad komponenty,
// signály, For a view! makro.
use leptos::prelude::*;

// Duration používame na určenie, ako dlho má toast správa zostať zobrazená.
use std::time::Duration;

// spawn_local použijeme na spustenie async kódu na klientovi.
// Tu ho používame na automatické odstránenie toastu po pár sekundách.
use leptos::task::spawn_local;

// sleep použijeme ako jednoduché čakanie v async kóde.
use gloo_timers::future::sleep;

// Enum určuje typ toast správy.
// Podľa typu potom vieme meniť vzhľad správy, napríklad farbu alebo ikonu.
#[derive(Clone, PartialEq)]
pub enum ToastType {
    Success,
    Error,
    Info,
}

// Struct reprezentuje jednu konkrétnu toast správu.
// Každá správa má ID, text a typ.
#[derive(Clone)]
pub struct ToastMessage {
    // ID používame na rozlíšenie jednotlivých toastov.
    // Je dôležité hlavne pri For cykle a pri odstraňovaní správy.
    pub id: usize,

    // Text, ktorý sa zobrazí používateľovi.
    pub message: String,

    // Typ správy, napríklad Success, Error alebo Info.
    pub toast_type: ToastType,
}

// Toaster je služba / helper, cez ktorý vieme pridávať toast správy.
// Je Copy a Clone, takže sa dá jednoducho posúvať medzi komponentmi.
#[derive(Copy, Clone)]
pub struct Toaster {
    // Zoznam aktuálne zobrazených toast správ.
    toasts: RwSignal<Vec<ToastMessage>>,

    // next_id drží ID, ktoré sa použije pre ďalší nový toast.
    next_id: RwSignal<usize>,
}

impl Toaster {
    // Vytvorí nový prázdny toaster.
    // Na začiatku nemá žiadne správy a next_id začína od 0.
    pub fn new() -> Self {
        Self {
            toasts: RwSignal::new(Vec::new()),
            next_id: RwSignal::new(0),
        }
    }

    // Univerzálna funkcia na pridanie novej toast správy.
    // Používajú ju potom error(), success() a info().
    pub fn add(&self, message: impl Into<String>, toast_type: ToastType) {
        // Zoberieme aktuálne ID bez sledovania reaktivity.
        // get_untracked znamená, že táto hodnota nespôsobí reaktívne sledovanie.
        let id = self.next_id.get_untracked();

        // Zvýšime next_id, aby ďalší toast dostal nové unikátne ID.
        self.next_id.update(|n| *n += 1);

        // Pridáme nový toast do zoznamu správ.
        self.toasts.update(|t| {
            t.push(ToastMessage {
                id,
                message: message.into(),
                toast_type,
            });
        });

        // Skopírujeme si signál toastov, aby sme ho mohli použiť v async bloku.
        let toasts_signal = self.toasts;

        // Spustíme async úlohu, ktorá po 4 sekundách toast automaticky odstráni.
        spawn_local(async move {
            // Počkáme 4 sekundy.
            sleep(Duration::from_secs(4)).await;

            // Zo zoznamu necháme iba tie správy, ktoré nemajú dané ID.
            // Takto odstránime iba konkrétny toast.
            toasts_signal.update(|t| t.retain(|msg| msg.id != id));
        });
    }

    // Skratka na zobrazenie chybovej toast správy.
    pub fn error(&self, message: impl Into<String>) {
        self.add(message, ToastType::Error);
    }

    // Skratka na zobrazenie úspešnej toast správy.
    pub fn success(&self, message: impl Into<String>) {
        self.add(message, ToastType::Success);
    }

    // Skratka na zobrazenie informačnej toast správy.
    pub fn info(&self, message: impl Into<String>) {
        self.add(message, ToastType::Info);
    }
}

// Tento komponent vykresľuje všetky aktuálne toast správy na stránke.
// Samotné správy sa pridávajú cez Toaster.
#[component]
pub fn ToastContainer() -> impl IntoView {
    // Z contextu si vytiahneme toaster.
    // Musí byť predtým poskytnutý v aplikácii cez provide_context.
    let toaster = expect_context::<Toaster>();

    view! {
        // Kontajner pre všetky toast správy.
        <div class="toast-container">
            // For prechádza všetky aktuálne toast správy.
            // Keď sa zoznam zmení, Leptos vie podľa key efektívne aktualizovať UI.
            <For
                each=move || toaster.toasts.get()
                key=|toast| toast.id
                children=move |toast| {
                    // Podľa typu toastu nastavíme CSS triedu.
                    // Tá potom určuje napríklad farbu správy.
                    let type_class = match toast.toast_type {
                        ToastType::Success => "toast-success",
                        ToastType::Error => "toast-error",
                        ToastType::Info => "toast-info",
                    };

                    // Podľa typu toastu nastavíme aj ikonu.
                    let icon = match toast.toast_type {
                        ToastType::Success => "✓",
                        ToastType::Error => "✕",
                        ToastType::Info => "ℹ",
                    };

                    view! {
                        // Jeden konkrétny toast.
                        // Trieda sa skladá zo základnej triedy toast a triedy podľa typu.
                        <div class=format!("toast {}", type_class)>
                            // Ikona toast správy.
                            <div class="toast-icon">{icon}</div>

                            // Text toast správy.
                            <div class="toast-content">{toast.message}</div>
                        </div>
                    }
                }
            />
        </div>
    }
}