// Importujeme základné veci z Leptosu, ktoré používame na komponenty, signály a view makro.
use leptos::prelude::*;

// spawn_local použijeme na spustenie async kódu na klientovi, napríklad pri upvote.
use leptos::task::spawn_local;

// Import serverových funkcií a dátových typov pre komentáre.
use crate::server::tasks::{get_comments, AddComment, CommentDto, upvote_comment};

// Toaster používame na zobrazovanie úspešných alebo chybových hlášok používateľovi.
use crate::components::toast::Toaster;

// Tento struct predstavuje komentár vo frontende.
// Je trochu upravený oproti tomu, čo príde zo servera, lebo obsahuje aj replies ako vnorené komentáre.
#[derive(Clone)]
pub struct Comment {
    pub id: i64,
    pub author: String,
    pub avatar: String,
    pub content: String,
    pub time_ago: String,
    pub upvotes: i32,

    // Tu sú uložené odpovede na daný komentár.
    // Vec<Comment> znamená zoznam ďalších komentárov.
    pub replies: Vec<Comment>,
}

// Táto funkcia z plochého zoznamu komentárov vytvorí strom komentárov.
// Zo servera prídu komentáre väčšinou ako jeden zoznam, kde každý komentár má parent_id.
// Ak je parent_id None, je to hlavný komentár. Ak má parent_id hodnotu, je to odpoveď.
fn build_comment_tree(flat_comments: &[CommentDto], parent_id: Option<i64>) -> Vec<Comment> {
    flat_comments
        .iter()
        // Vyberieme iba tie komentáre, ktoré patria pod aktuálneho rodiča.
        .filter(|c| c.parent_id == parent_id)
        // Každý CommentDto zo servera premeníme na náš frontendový Comment.
        .map(|c| Comment {
            id: c.id,
            author: c.author_name.clone(),

            // Avatar je len prvé písmeno mena autora.
            // unwrap_or('?') je poistka, keby meno bolo prázdne.
            avatar: c.author_name.chars().next().unwrap_or('?').to_string().to_uppercase(),

            content: c.content.clone(),
            time_ago: c.created_at.clone(),
            upvotes: c.upvotes,

            // Rekurzívne nájdeme odpovede na tento komentár.
            // To znamená, že funkcia volá samu seba pre ďalšiu úroveň odpovedí.
            replies: build_comment_tree(flat_comments, Some(c.id)),
        })
        // Výsledok mapovania pozbierame do Vec<Comment>.
        .collect()
}

// Komponent pre zobrazenie jedného komentára.
// Tento komponent sa používa aj pre odpovede, preto sa nižšie volá rekurzívne.
#[component]
pub fn CommentItem(
    comment: Comment,

    // Signál, ktorým nastavujeme, na ktorý komentár používateľ práve odpovedá.
    set_replying_to: WriteSignal<Option<(i64, String)>>,

    // Signál, ktorým vieme vynútiť opätovné načítanie komentárov.
    set_reload_trigger: WriteSignal<i32>,
) -> impl IntoView {
    // Z contextu si vytiahneme toaster, aby sme vedeli zobraziť chybu pri upvote.
    let toaster = expect_context::<Toaster>();

    // ID komentára si uložíme bokom, aby sme ho mohli použiť v closure.
    let comment_id = comment.id;

    // Funkcia, ktorá sa spustí po kliknutí na upvote tlačidlo.
    let handle_upvote = move |_| {
        let toaster = toaster.clone();

        // Serverová funkcia je async, preto ju spustíme cez spawn_local.
        spawn_local(async move {
            match upvote_comment(comment_id).await {
                Ok(_) => {
                    // Ak upvote prebehne úspešne, zvýšime reload_trigger.
                    // Tým povieme Resource, aby znovu načítal komentáre.
                    set_reload_trigger.update(|n| *n += 1);
                }
                Err(_) => {
                    // Ak nastane chyba, zobrazíme používateľovi hlášku.
                    toaster.error("Failed to upvote comment");
                }
            }
        });
    };

    // Meno autora si naklonujeme, aby sme ho mohli použiť v reply handleri.
    let author_name = comment.author.clone();

    // Funkcia, ktorá sa spustí po kliknutí na Reply.
    let handle_reply = move |_| {
        // Uložíme ID komentára a meno autora, na ktorého odpovedáme.
        set_replying_to.set(Some((comment_id, author_name.clone())));
    };

    view! {
        <div class="comment-thread">
            <div class="comment-main">
                <div class="comment-left">
                    // Avatar komentára, napríklad prvé písmeno mena autora.
                    <div class="comment-avatar">{comment.avatar.clone()}</div>

                    // Ak komentár má odpovede, zobrazíme čiaru vlákna.
                    // Je to len vizuálna pomôcka, aby bolo vidno vnorené komentáre.
                    {if !comment.replies.is_empty() {
                        view! { <div class="thread-line"></div> }.into_any()
                    } else {
                        view! { <div></div> }.into_any()
                    }}
                </div>

                <div class="comment-body">
                    <div class="comment-header">
                        // Meno autora komentára.
                        <span class="comment-author">{comment.author.clone()}</span>

                        // Čas vytvorenia komentára, napríklad "2 hours ago".
                        <span
                            class="comment-time"
                            style="color: #94a3b8; font-size: 0.8rem; margin-left: 8px;"
                        >
                            {comment.time_ago.clone()}
                        </span>
                    </div>

                    // Text samotného komentára.
                    <div class="comment-content">{comment.content.clone()}</div>

                    <div class="comment-actions">
                        // Tlačidlo na upvote komentára.
                        <button class="action-btn" on:click=handle_upvote>
                            "↑ " {comment.upvotes} " Upvotes"
                        </button>

                        // Tlačidlo na odpoveď na komentár.
                        <button class="action-btn reply-btn" on:click=handle_reply>
                            "Reply"
                        </button>
                    </div>
                </div>
            </div>

            <div class="comment-replies">
                // Tu vykresľujeme všetky odpovede na tento komentár.
                // Každá odpoveď je znovu CommentItem, čiže komponent volá sám seba.
                {comment.replies.into_iter().map(|reply| {
                    view! {
                        <CommentItem
                            comment=reply
                            set_replying_to=set_replying_to
                            set_reload_trigger=set_reload_trigger
                        />
                    }
                }).collect_view()}
            </div>
        </div>
    }
}

// Hlavný komponent celej diskusie k úlohe.
// Dostane task_id a podľa neho načíta komentáre patriace ku konkrétnej úlohe.
#[component]
pub fn IssueDiscussion(#[prop(into)] task_id: Signal<i64>) -> impl IntoView {
    // Toaster použijeme na hlášky po pridaní komentára alebo pri chybe.
    let toaster = expect_context::<Toaster>();

    // ServerAction slúži na odoslanie formulára na server.
    // V tomto prípade ide o pridanie nového komentára.
    let add_action = ServerAction::<AddComment>::new();

    // Tu sledujeme výsledok serverovej akcie.
    let add_result = add_action.value();

    // reload_trigger používame ako jednoduchý trik na refresh komentárov.
    // Keď sa jeho hodnota zmení, Resource znovu načíta dáta.
    let (reload_trigger, set_reload_trigger) = signal(0);

    // replying_to drží informáciu, či práve odpovedáme na nejaký komentár.
    // None znamená, že píšeme nový hlavný komentár.
    // Some((id, meno)) znamená, že odpovedáme na konkrétny komentár.
    let (replying_to, set_replying_to) = signal::<Option<(i64, String)>>(None);

    // Tento Effect reaguje na výsledok pridania komentára.
    // Spustí sa, keď serverová akcia vráti výsledok.
    Effect::new(move |_| {
        if let Some(res) = add_result.get() {
            match res {
                Ok(_) => {
                    // Ak sa komentár pridal úspešne, ukážeme hlášku.
                    toaster.success("Comment added successfully");

                    // Vyčistíme stav serverovej akcie.
                    add_action.clear();

                    // Zrušíme reply mód, aby ďalší komentár nebol omylom odpoveďou.
                    set_replying_to.set(None);

                    // Vynútime znovunačítanie komentárov.
                    set_reload_trigger.update(|n| *n += 1);
                }
                Err(e) => {
                    // Chybovú správu zo servera trochu očistíme, aby bola čitateľnejšia.
                    let err_msg = e
                        .to_string()
                        .replace("error running server function:", "")
                        .trim()
                        .to_string();

                    // Zobrazíme chybu používateľovi.
                    toaster.error(format!("Failed to add comment: {}", err_msg));
                }
            }
        }
    });

    // Resource automaticky načítava komentáre zo servera.
    // Závisí od task_id a reload_trigger, čiže sa obnoví pri zmene úlohy alebo po pridaní/upvote.
    let comments_resource = Resource::new(
        move || (task_id.get(), reload_trigger.get()),
        |(id, _)| async move {
            // Ak je id 0, nebudeme nič načítavať.
            // Je to pravdepodobne ochrana pred neplatným alebo ešte nenačítaným task_id.
            if id == 0 {
                return vec![];
            }

            // Načítame komentáre zo servera.
            // unwrap_or_default znamená, že pri chybe sa použije prázdny zoznam.
            get_comments(id).await.unwrap_or_default()
        },
    );

    view! {
        <section class="discussion-section">
            <h3 class="discussion-title">"Discussion"</h3>

            // Formulár na pridanie nového komentára alebo odpovede.
            <ActionForm action=add_action attr:class="new-comment-box">
                // Skryté pole s ID úlohy, ku ktorej patrí komentár.
                <input type="hidden" name="task_id" value=move || task_id.get() />

                // Ak odpovedáme na existujúci komentár, pošleme serveru aj parent_id.
                // Server potom vie, že nový komentár má byť odpoveď.
                <Show when=move || replying_to.get().is_some()>
                    <input
                        type="hidden"
                        name="parent_id"
                        value=move || replying_to.get().unwrap().0
                    />
                </Show>

                // Avatar aktuálneho používateľa pri písaní komentára.
                <div class="comment-avatar">"Me"</div>

                <div class="new-comment-input-wrap">
                    // Ak sme v reply móde, zobrazíme informáciu, komu odpovedáme.
                    <Show when=move || replying_to.get().is_some()>
                        <div style="font-size: 0.8rem; color: #3b82f6; margin-bottom: 4px; display: flex; justify-content: space-between;">
                            <span>
                                "Replying to "
                                <strong>{move || replying_to.get().unwrap().1}</strong>
                            </span>

                            // Kliknutím na Cancel zrušíme odpovedanie.
                            <span
                                style="cursor: pointer; text-decoration: underline;"
                                on:click=move |_| set_replying_to.set(None)
                            >
                                "Cancel"
                            </span>
                        </div>
                    </Show>

                    // Textové pole, kam používateľ napíše komentár.
                    <textarea
                        name="content"
                        placeholder="Add a comment... (Markdown supported)"
                        required
                    ></textarea>

                    <div class="new-comment-actions">
                        // Submit tlačidlo odošle komentár na server.
                        // Počas odosielania je tlačidlo disabled, aby sa komentár neodoslal viackrát.
                        <button
                            type="submit"
                            class="primary-button"
                            style="height: 32px; font-size: 0.8rem;"
                            disabled=add_action.pending()
                        >
                            "Comment"
                        </button>
                    </div>
                </div>
            </ActionForm>

            <div class="threads-container">
                // Transition zobrazí fallback počas načítavania komentárov.
                <Transition fallback=move || {
                    view! { <div>"Loading discussion..."</div> }
                }>
                    {move || {
                        // Z Resource si zoberieme komentáre.
                        // Ak ešte nie sú načítané, použijeme prázdny zoznam.
                        let db_comments = comments_resource.get().unwrap_or_default();

                        if db_comments.is_empty() {
                            // Keď ešte nie sú žiadne komentáre, zobrazíme jednoduchú správu.
                            view! {
                                <p style="color: #64748b; margin-top: 1rem;">
                                    "No comments yet. Start the discussion!"
                                </p>
                            }
                            .into_any()
                        } else {
                            // Zo zoznamu komentárov vytvoríme strom, aby sa odpovede zobrazili vnorené.
                            let comment_tree = build_comment_tree(&db_comments, None);

                            // Každý hlavný komentár vykreslíme ako CommentItem.
                            comment_tree
                                .into_iter()
                                .map(|c| {
                                    view! {
                                        <CommentItem
                                            comment=c
                                            set_replying_to=set_replying_to
                                            set_reload_trigger=set_reload_trigger
                                        />
                                    }
                                })
                                .collect_view()
                                .into_any()
                        }
                    }}
                </Transition>
            </div>
        </section>
    }
}