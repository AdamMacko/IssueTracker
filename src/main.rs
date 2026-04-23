#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use issue_tracker::app::*;
    use sqlx::sqlite::SqlitePoolOptions;
    use crate::ax_state::AppState;

    // 1. Načítanie konfigurácie z .env súboru
    let _ = dotenvy::dotenv();

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);

    // 2. Pripojenie k SQLite databáze
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
    
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Could not connect to SQLite database");

    // 3. Automatické vytvorenie tabuľky (ak ešte neexistuje)
    // Toto spustíme hneď po pripojení, aby sme mali istotu, že máme kam ukladať
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS projects (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            description TEXT NOT NULL
        );
        "#
    )
    .execute(&pool)
    .await
    .expect("Failed to initialize database tables");

    log!("Database is initialized and 'projects' table is ready.");

    // 4. Vytvorenie stavu aplikácie (AppState)
    let state = AppState {
        leptos_options: leptos_options.clone(),
        pool,
    };

    // 5. Nastavenie Routera
    let app = Router::new()
        .leptos_routes(&state, routes, {
            let state = state.clone();
            move || shell(state.leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler::<AppState, _>(shell))
        .with_state(state);

    // 6. Spustenie servera
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

// Pomocný modul pre zdieľaný stav (State management)
#[cfg(feature = "ssr")]
mod ax_state {
    use axum::extract::FromRef;
    use leptos::prelude::LeptosOptions;
    use sqlx::SqlitePool;

    #[derive(FromRef, Clone, Debug)]
    pub struct AppState {
        pub leptos_options: LeptosOptions,
        pub pool: SqlitePool,
    }
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // Na strane klienta (WASM) nepotrebujeme main, 
    // hydratácia prebieha cez lib.rs
}