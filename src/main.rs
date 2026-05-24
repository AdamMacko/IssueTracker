// Tento main sa skompiluje iba vtedy, keď je zapnutý feature "ssr".
// SSR znamená Server Side Rendering, čiže aplikácia beží aj na serveri.
#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    // Importujeme vlastný AppState, kde máme uložené globálne veci aplikácie.
    use ax_state::AppState;

    // Axum používame ako webový server.
    // Router definuje routy a Extension slúži na pridanie zdieľaných dát, napríklad databázy.
    use axum::{Router, Extension};

    // Importujeme App a shell z našej Leptos aplikácie.
    use issue_tracker::app::*;

    // log používame na jednoduché vypisovanie správ do konzoly.
    use leptos::logging::log;

    // Importujeme základné veci z Leptosu, napríklad konfiguráciu a provide_context.
    use leptos::prelude::*;

    // Funkcie z leptos_axum, ktoré prepájajú Leptos s Axum serverom.
    use leptos_axum::{generate_route_list, LeptosRoutes};

    // SqlitePoolOptions používame na vytvorenie poolu pripojení k SQLite databáze.
    use sqlx::sqlite::SqlitePoolOptions;

    // Funkcia na vloženie testovacích dát do databázy.
    use issue_tracker::server::seed::seed_database;

    // Načítame premenné z .env súboru.
    // Napríklad DATABASE_URL alebo SEED_DB.
    let _ = dotenvy::dotenv();

    // Načítame konfiguráciu Leptos aplikácie.
    // unwrap() znamená, že ak konfigurácia zlyhá, aplikácia spadne.
    let conf = get_configuration(None).unwrap();

    // Adresa, na ktorej bude server počúvať.
    let addr = conf.leptos_options.site_addr;

    // Leptos options si uložíme bokom, lebo ich budeme používať aj v state.
    let leptos_options = conf.leptos_options;

    // Vygenerujeme zoznam rout z App komponentu.
    // Leptos/Axum podľa toho vie, aké stránky aplikácia obsahuje.
    let routes = generate_route_list(App);

    // Z .env načítame DATABASE_URL.
    // Ak tam nie je, aplikácia skončí s chybou.
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");

    // Vytvoríme connection pool k SQLite databáze.
    // Pool znamená, že aplikácia môže zdieľať viac databázových pripojení.
    let pool = SqlitePoolOptions::new()
        // Nastavíme maximálne 5 pripojení naraz.
        .max_connections(5)
        // Pripojíme sa na databázu podľa DATABASE_URL.
        .connect(&db_url)
        .await
        .expect("Could not connect to SQLite database");

    // Spustíme databázové migrácie.
    // Migrácie vytvoria alebo upravia tabuľky podľa súborov v priečinku migrations.
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run database migrations");

    // Ak je v .env nastavená premenná SEED_DB,
    // vložíme do databázy testovacie dáta.
    if std::env::var("SEED_DB").is_ok() {
        log!("Seeding database with test data...");
        seed_database(&pool).await.expect("Failed to seed database");
    }

    // Vypíšeme do konzoly, že databáza je pripravená.
    log!("Database is initialized and migrations are done.");

    // Vytvoríme globálny stav aplikácie.
    // Bude obsahovať Leptos nastavenia a databázový pool.
    let state = AppState {
        leptos_options: leptos_options.clone(),
        pool: pool.clone(),
    };

    // Vytvoríme Axum router, čiže hlavný server aplikácie.
    let app = Router::new()
        // Tu napájame Leptos routy na Axum.
        .leptos_routes(&state, routes, {
            // State si naklonujeme, aby sa dal použiť v closure.
            let state = state.clone();

            move || {
                // Databázový pool dáme do Leptos contextu.
                // Vďaka tomu ho môžu serverové funkcie alebo komponenty získať cez context.
                provide_context(state.pool.clone());

                // Vrátime HTML shell aplikácie.
                shell(state.leptos_options.clone())
            }
        })
        // Fallback rieši statické súbory a chybové stránky.
        // Napríklad CSS, JS alebo 404.
        .fallback(leptos_axum::file_and_error_handler::<AppState, _>(shell))
        // Pridáme databázový pool ako Axum Extension.
        // Serverové funkcie si ho potom vedia vytiahnuť cez extract.
        .layer(Extension(pool.clone()))
        // Pridáme AppState do routera.
        .with_state(state);

    // Vypíšeme adresu, na ktorej server beží.
    log!("listening on http://{}", &addr);

    // Vytvoríme TCP listener na danej adrese.
    // Ten čaká na HTTP požiadavky od používateľov.
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    // Spustíme Axum server.
    // app.into_make_service() premení router na službu, ktorú vie Axum obsluhovať.
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

// Tento modul existuje iba pri SSR.
// Obsahuje AppState, teda spoločný stav aplikácie na serveri.
#[cfg(feature = "ssr")]
mod ax_state {
    // FromRef umožňuje Axumu vytiahnuť konkrétne časti zo state.
    use axum::extract::FromRef;

    // LeptosOptions obsahuje nastavenia Leptos aplikácie.
    use leptos::prelude::LeptosOptions;

    // SqlitePool je pool pripojení k SQLite databáze.
    use sqlx::SqlitePool;

    // AppState drží globálne veci, ktoré server potrebuje.
    // Clone je potrebné, lebo state sa používa na viacerých miestach.
    // Debug umožňuje jednoduchšie vypisovanie pri debugovaní.
    #[derive(FromRef, Clone, Debug)]
    pub struct AppState {
        // Nastavenia Leptos aplikácie.
        pub leptos_options: LeptosOptions,

        // Databázový pool, cez ktorý server komunikuje s databázou.
        pub pool: SqlitePool,
    }
}

// Keď aplikácia nebeží so SSR feature, použije sa tento prázdny main.
// Je to potrebné, aby sa frontendová časť vedela skompilovať aj bez serverového mainu.
#[cfg(not(feature = "ssr"))]
pub fn main() {}