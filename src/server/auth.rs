// Importujeme základné veci z Leptosu.
// Tu hlavne potrebujeme #[server] a ServerFnError.
use leptos::prelude::*;

// Serde používame na serializáciu a deserializáciu dát.
// To je potrebné, keď sa dáta posielajú medzi serverom a klientom.
use serde::{Deserialize, Serialize};

// Serverová funkcia na registráciu používateľa.
// Dostane username, email a password z registračného formulára.
#[server]
pub async fn register_user(
    username: String,
    email: String,
    password: String,
) -> Result<(), ServerFnError> {
    // Tento blok sa skompiluje iba na serveri.
    // Databáza a hashovanie hesla sa nemajú vykonávať v prehliadači.
    #[cfg(feature = "ssr")]
    {
        // bcrypt používame na zahashovanie hesla.
        // DEFAULT_COST je predvolená náročnosť hashovania.
        use bcrypt::{hash, DEFAULT_COST};

        // Získame databázový pool z Axum extension.
        // Pool je pripojenie alebo skupina pripojení k SQLite databáze.
        let axum::Extension(pool) =
            leptos_axum::extract::<axum::Extension<sqlx::SqlitePool>>()
                .await
                .map_err(|e| ServerFnError::new(e.to_string()))?;

        // Heslo nikdy neukladáme ako čistý text.
        // Najprv ho zahashujeme pomocou bcryptu.
        let hashed_password = hash(password, DEFAULT_COST)
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        // Vložíme nového používateľa do tabuľky users.
        // Do databázy ukladáme username, email a zahashované heslo.
        sqlx::query(
            "INSERT INTO users (username, email, password_hash) VALUES (?, ?, ?)"
        )
        .bind(username)
        .bind(email)
        .bind(hashed_password)
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

        // Ak všetko prešlo bez chyby, vrátime Ok.
        Ok(())
    }

    // Tento blok sa používa, keď kód nebeží na serveri.
    // Len aby sa funkcia vedela skompilovať aj na klientovi.
    #[cfg(not(feature = "ssr"))]
    {
        let _ = (username, email, password);
        Ok(())
    }
}

// Serverová funkcia na prihlásenie používateľa.
// Dostane email a password z login formulára.
#[server]
pub async fn login_user(email: String, password: String) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use axum::Extension;
        use sqlx::SqlitePool;
        use bcrypt::verify;
        use leptos_axum::{ResponseOptions, redirect};
        use axum::http::header::{SET_COOKIE, HeaderValue};
        use uuid::Uuid;

        // Získame databázový pool.
        let Extension(pool) = leptos_axum::extract::<Extension<SqlitePool>>()
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        #[derive(sqlx::FromRow)]
        struct UserAuth {
            id: i64,
            password_hash: String,
        }

        // Nájdeme používateľa podľa emailu.
        let user = sqlx::query_as::<_, UserAuth>(
            "SELECT id, password_hash FROM users WHERE email = ?"
        )
        .bind(&email)
        .fetch_optional(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

        if let Some(user) = user {
            // Overíme heslo pomocou bcryptu.
            let is_valid = verify(&password, &user.password_hash)
                .map_err(|_| ServerFnError::new("Internal auth error"))?;

            if is_valid {
                // Vygenerujeme náhodné session_id.
                // Toto pôjde do cookie namiesto user_id.
                let session_id = Uuid::new_v4().to_string();

                // Session uložíme do databázy.
                // Platnosť nastavíme na 24 hodín.
                sqlx::query(
                    "INSERT INTO sessions (id, user_id, expires_at) VALUES (?, ?, datetime('now', '+1 day'))"
                )
                .bind(&session_id)
                .bind(user.id)
                .execute(&pool)
                .await
                .map_err(|e| ServerFnError::new(e.to_string()))?;

                let response_options = expect_context::<ResponseOptions>();

                // Do cookie už nedávame user_id.
                // Dávame tam iba náhodné session_id.
                let cookie_str = format!(
                    "session_id={}; Path=/; HttpOnly; Max-Age=86400; SameSite=Lax",
                    session_id
                );

                response_options.insert_header(
                    SET_COOKIE,
                    HeaderValue::from_str(&cookie_str).expect("Invalid cookie header"),
                );

                // Po úspešnom prihlásení presmerujeme používateľa.
                redirect("/dashboard");

                Ok(())
            } else {
                Err(ServerFnError::new("Invalid email or password."))
            }
        } else {
            Err(ServerFnError::new("Invalid email or password."))
        }
    }

    #[cfg(not(feature = "ssr"))]
    {
        let _ = (email, password);
        Ok(())
    }
}

// Serverová funkcia, ktorá zistí aktuálne prihláseného používateľa.
// Vráti Some(user_id), ak je používateľ prihlásený.
// Vráti None, ak nie je prihlásený.
#[server]
pub async fn get_current_user() -> Result<Option<i64>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use axum::http::HeaderMap;
        use axum::Extension;
        use sqlx::SqlitePool;

        // Z requestu vytiahneme HTTP hlavičky.
        let headers = leptos_axum::extract::<HeaderMap>()
            .await
            .map_err(|_| ServerFnError::new("Error reading headers"))?;

        // Získame databázový pool.
        let Extension(pool) = leptos_axum::extract::<Extension<SqlitePool>>()
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        // Sem uložíme session_id z cookie.
        let mut session_id: Option<String> = None;

        // Prečítame Cookie header.
        if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
            if let Ok(cookie_str) = cookie_header.to_str() {
                for cookie in cookie_str.split(';') {
                    let cookie = cookie.trim();

                    // Hľadáme cookie session_id.
                    if let Some(value) = cookie.strip_prefix("session_id=") {
                        session_id = Some(value.to_string());
                        break;
                    }
                }
            }
        }

        // Ak cookie session_id neexistuje, používateľ nie je prihlásený.
        let Some(session_id) = session_id else {
            return Ok(None);
        };

        // Voliteľne zmažeme expirované sessions.
        // Nie je to nutné pre funkčnosť, ale udržiava to tabuľku čistejšiu.
        let _ = sqlx::query("DELETE FROM sessions WHERE expires_at <= datetime('now')")
            .execute(&pool)
            .await;

        // Podľa session_id nájdeme user_id.
        // Zároveň kontrolujeme, či session ešte neexpirovala.
        let user_id = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT user_id
            FROM sessions
            WHERE id = ?
              AND expires_at > datetime('now')
            "#
        )
        .bind(session_id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

        Ok(user_id)
    }

    #[cfg(not(feature = "ssr"))]
    {
        Ok(None)
    }
}

// Struct s verejnými údajmi používateľa.
// Tento typ sa posiela na frontend, preto obsahuje iba bezpečné údaje.
// Heslo ani password_hash tu určite nemajú byť.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct UserProfile {
    pub id: i64,
    pub username: String,
    pub email: String,
}

// Serverová funkcia na načítanie profilu aktuálne prihláseného používateľa.
#[server]
pub async fn get_user_profile() -> Result<Option<UserProfile>, ServerFnError> {
    // Najprv zistíme, kto je aktuálne prihlásený.
    let user_id = get_current_user().await?;

    if let Some(id) = user_id {
        #[cfg(feature = "ssr")]
        {
            use axum::Extension;
            use sqlx::SqlitePool;

            // Získame databázový pool.
            let Extension(pool) = leptos_axum::extract::<Extension<SqlitePool>>()
                .await
                .map_err(|e| ServerFnError::new(e.to_string()))?;

            // Načítame používateľa podľa ID.
            // Vyberáme iba id, username a email.
            let user = sqlx::query_as::<_, UserProfile>(
                "SELECT id, username, email FROM users WHERE id = ?"
            )
            .bind(id)
            .fetch_optional(&pool)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

            // Vrátime profil používateľa.
            Ok(user)
        }

        // Klientsky fallback.
        #[cfg(not(feature = "ssr"))]
        Ok(None)
    } else {
        // Ak nikto nie je prihlásený, nevrátime žiadny profil.
        Ok(None)
    }
}

// Serverová funkcia na odhlásenie používateľa.
#[server]
pub async fn logout_user() -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use axum::http::HeaderMap;
        use axum::Extension;
        use sqlx::SqlitePool;
        use leptos_axum::{ResponseOptions, redirect};
        use axum::http::header::{SET_COOKIE, HeaderValue};

        let response_options = expect_context::<ResponseOptions>();

        // Prečítame hlavičky, aby sme vedeli získať session_id z cookie.
        let headers = leptos_axum::extract::<HeaderMap>()
            .await
            .map_err(|_| ServerFnError::new("Error reading headers"))?;

        // Získame databázový pool.
        let Extension(pool) = leptos_axum::extract::<Extension<SqlitePool>>()
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        let mut session_id: Option<String> = None;

        // Nájdeme session_id v cookie.
        if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
            if let Ok(cookie_str) = cookie_header.to_str() {
                for cookie in cookie_str.split(';') {
                    let cookie = cookie.trim();

                    if let Some(value) = cookie.strip_prefix("session_id=") {
                        session_id = Some(value.to_string());
                        break;
                    }
                }
            }
        }

        // Ak session existuje, zmažeme ju z databázy.
        if let Some(session_id) = session_id {
            sqlx::query("DELETE FROM sessions WHERE id = ?")
                .bind(session_id)
                .execute(&pool)
                .await
                .map_err(|e| ServerFnError::new(e.to_string()))?;
        }

        // Zmažeme novú session_id cookie.
        let session_cookie = "session_id=; Path=/; HttpOnly; Max-Age=0; SameSite=Lax";

        response_options.insert_header(
            SET_COOKIE,
            HeaderValue::from_str(session_cookie).expect("Invalid cookie header"),
        );

        // Pre istotu zmažeme aj starú user_id cookie,
        // keby ešte zostala v prehliadači z predchádzajúcej verzie.
        let old_user_cookie = "user_id=; Path=/; HttpOnly; Max-Age=0; SameSite=Lax";

        response_options.insert_header(
            SET_COOKIE,
            HeaderValue::from_str(old_user_cookie).expect("Invalid cookie header"),
        );

        redirect("/login");
    }

    Ok(())
}

// DTO pre výsledok vyhľadávania používateľov.
// DTO znamená jednoduchý dátový objekt na prenos medzi serverom a klientom.
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct UserSearchDto {
    pub id: i64,
    pub username: String,
    pub email: String,
}

// Serverová funkcia na vyhľadávanie používateľov.
// Používa sa napríklad pri pozývaní členov do projektu.
#[server]
pub async fn search_users(query: String) -> Result<Vec<UserSearchDto>, ServerFnError> {
    // Zistíme ID aktuálne prihláseného používateľa.
    // unwrap_or(0) znamená, že ak nikto nie je prihlásený, použije sa 0.
    let current_user_id = get_current_user().await?.unwrap_or(0);

    #[cfg(feature = "ssr")]
    {
        use axum::Extension;
        use sqlx::SqlitePool;

        // Získame databázový pool.
        let Extension(pool) = leptos_axum::extract::<Extension<SqlitePool>>()
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        // Pripravíme LIKE pattern pre SQL vyhľadávanie.
        // Napríklad query "adam" sa zmení na "%adam%".
        let search_pattern = format!("%{}%", query);

        // Vyhľadáme používateľov podľa username alebo emailu.
        // Zároveň vylúčime aktuálne prihláseného používateľa,
        // aby sám seba nevidel v zozname na pozvanie.
        let users = sqlx::query_as::<_, UserSearchDto>(
            "SELECT id, username, email FROM users WHERE id != ? AND (username LIKE ? OR email LIKE ?) LIMIT 15"
        )
        .bind(current_user_id)
        .bind(&search_pattern)
        .bind(&search_pattern)
        .fetch_all(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

        // Vrátime nájdených používateľov.
        Ok(users)
    }

    // Klientsky fallback.
    #[cfg(not(feature = "ssr"))]
    {
        let _ = query;
        Ok(vec![])
    }
}