// Importujeme základné veci z Leptosu.
// Tu potrebujeme hlavne #[server] a ServerFnError.
use leptos::prelude::*;

// Serde používame na serializáciu a deserializáciu dát.
// Je to potrebné, keď sa structy posielajú medzi serverom a frontendom.
use serde::{Deserialize, Serialize};

// Funkcia na zistenie aktuálne prihláseného používateľa.
use crate::server::auth::get_current_user;

// Struct reprezentuje jeden projekt.
// Tento typ sa používa na posielanie projektu zo servera na frontend.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub project_key: String,
    pub description: String,
}

// Struct pre štatistiky člena tímu.
// Používa sa napríklad na dashboarde pri grafe odpracovaného času.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeamMemberStats {
    pub name: String,
    pub hours: f32,
    pub color: String,
}

// Serverová funkcia na načítanie projektov aktuálne prihláseného používateľa.
#[server]
pub async fn get_projects() -> Result<Vec<Project>, ServerFnError> {
    // Najprv zistíme, kto je prihlásený.
    // Ak používateľ nie je prihlásený, vrátime chybu Unauthorized.
    let user_id = get_current_user()
        .await?
        .ok_or_else(|| ServerFnError::new("Unauthorized access. You must be signed in."))?;

    // Získame databázový pool z Axum extension.
    // Pool používame na vykonávanie SQL query.
    let axum::Extension(pool) =
        leptos_axum::extract::<axum::Extension<sqlx::SqlitePool>>()
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

    // Načítame projekty, kde je používateľ buď owner,
    // alebo je členom projektu cez tabuľku project_members.
    let projects = sqlx::query_as::<_, Project>(
        r#"
        SELECT DISTINCT p.id, p.name, p.project_key, p.description 
        FROM projects p
        LEFT JOIN project_members pm ON p.id = pm.project_id
        WHERE p.owner_id = ? OR pm.user_id = ?
        ORDER BY p.id DESC
        "#,
    )
    .bind(user_id)
    .bind(user_id)
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    // Vrátime zoznam projektov.
    Ok(projects)
}

// Serverová funkcia na vytvorenie nového projektu.
#[server]
pub async fn create_project(
    name: String,
    project_key: String,
    description: String,
    invited_users_str: Option<String>,
) -> Result<(), ServerFnError> {
    // Zistíme aktuálneho používateľa.
    // Tento používateľ bude owner nového projektu.
    let user_id = get_current_user()
        .await?
        .ok_or_else(|| ServerFnError::new("Unauthorized access. You must be signed in."))?;

    // invited_users_str príde z formulára ako text, napríklad "1,5,8".
    // Tu ho rozdelíme podľa čiarky a prevedieme na čísla.
    let users_to_invite: Vec<i32> = invited_users_str
        .unwrap_or_default()
        .split(',')
        .filter_map(|s| s.trim().parse::<i32>().ok())
        .collect();

    // Reálna práca s databázou sa robí iba na serveri.
    #[cfg(feature = "ssr")]
    {
        use axum::Extension;
        use sqlx::SqlitePool;

        // Získame databázový pool.
        let Extension(pool) = leptos_axum::extract::<Extension<SqlitePool>>()
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        // Začneme databázovú transakciu.
        // To znamená, že buď sa vykoná všetko, alebo nič.
        let mut tx = pool
            .begin()
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        // Vložíme nový projekt do tabuľky projects.
        let result = sqlx::query(
            "INSERT INTO projects (name, project_key, description, owner_id) VALUES (?, ?, ?, ?)",
        )
        .bind(&name)
        .bind(&project_key)
        .bind(&description)
        .bind(user_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

        // Získame ID práve vytvoreného projektu.
        let project_id = result.last_insert_rowid();

        // Pridáme ownera aj do tabuľky project_members.
        // Vďaka tomu sa s členmi projektu pracuje jednotne.
        sqlx::query(
            "INSERT INTO project_members (project_id, user_id, role) VALUES (?, ?, 'owner')",
        )
        .bind(project_id)
        .bind(user_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

        // Prejdeme všetkých pozvaných používateľov.
        for invitee_id in users_to_invite {
            // Kontrolujeme, aby sme ownera nepridali ešte raz ako membera.
            if invitee_id != user_id as i32 {
                // Pridáme pozvaného používateľa ako člena projektu.
                sqlx::query(
                    "INSERT INTO project_members (project_id, user_id, role) VALUES (?, ?, 'member')",
                )
                .bind(project_id)
                .bind(invitee_id)
                .execute(&mut *tx)
                .await
                .map_err(|e| ServerFnError::new(e.to_string()))?;
            }
        }

        // Ak všetky inserty prešli úspešne, transakciu potvrdíme.
        tx.commit()
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        Ok(())
    }

    // Klientsky fallback, aby sa kód vedel skompilovať aj bez SSR.
    #[cfg(not(feature = "ssr"))]
    {
        let _ = (name, project_key, description, invited_users_str);
        Ok(())
    }
}

// Pomocný struct pre riadok zo štatistickej SQL query.
// Používa sa iba ako medzikrok pred vytvorením TeamMemberStats.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct DbStatRow {
    pub name: String,
    pub hours: f64,
}

// Serverová funkcia na načítanie štatistík projektu.
// Vracia odpracované hodiny jednotlivých členov tímu.
#[server]
pub async fn get_project_stats(project_id: i32) -> Result<Vec<TeamMemberStats>, ServerFnError> {
    // Overíme, že používateľ je prihlásený.
    let user_id = get_current_user()
        .await?
        .ok_or_else(|| ServerFnError::new("Unauthorized access. You must be signed in."))?;

    // Získame databázový pool.
    let axum::Extension(pool) =
        leptos_axum::extract::<axum::Extension<sqlx::SqlitePool>>()
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

    // Načítame štatistiky členov projektu.
    // Spočítame duration_seconds z time_entries a prevedieme ich na hodiny.
    let stats = sqlx::query_as::<_, DbStatRow>(
        r#"
        SELECT 
            u.username as name,
            COALESCE(SUM(te.duration_seconds) / 3600.0, 0.0) as hours
        FROM users u
        JOIN project_members pm ON u.id = pm.user_id
        LEFT JOIN tasks t ON t.project_id = pm.project_id
        LEFT JOIN time_entries te ON te.task_id = t.id AND te.user_id = u.id
        WHERE pm.project_id = ?
          AND EXISTS (
              SELECT 1 FROM project_members 
              WHERE project_id = ? AND user_id = ?
          )
        GROUP BY u.id
        "#,
    )
    .bind(project_id)
    .bind(project_id)
    .bind(user_id)
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    // Ak projekt nemá žiadne štatistiky, vrátime prázdny zoznam.
    if stats.is_empty() {
        return Ok(vec![]);
    }

    // Farby pre členov tímu v grafe.
    // Používajú sa opakovane podľa indexu.
    let colors = vec!["#6366F1", "#F59E0B", "#10B981", "#EF4444", "#8B5CF6"];

    // Premeníme databázové riadky na TeamMemberStats,
    // ktoré potom vie frontend jednoducho zobraziť.
    let result = stats
        .into_iter()
        .enumerate()
        .map(|(i, row)| TeamMemberStats {
            name: row.name,
            hours: row.hours as f32,

            // Farba sa vyberie podľa poradia člena.
            // Ak je členov viac ako farieb, farby sa začnú opakovať.
            color: colors[i % colors.len()].to_string(),
        })
        .collect();

    Ok(result)
}

// Serverová funkcia na načítanie jedného konkrétneho projektu podľa ID.
#[server]
pub async fn get_project(project_id: i32) -> Result<Project, ServerFnError> {
    // Overíme, že používateľ je prihlásený.
    let user_id = get_current_user()
        .await?
        .ok_or_else(|| ServerFnError::new("Unauthorized access."))?;

    // Získame databázový pool.
    let axum::Extension(pool) =
        leptos_axum::extract::<axum::Extension<sqlx::SqlitePool>>()
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

    // Načítame projekt iba vtedy, ak používateľ patrí medzi členov projektu.
    // EXISTS tu slúži ako kontrola oprávnenia.
    let project = sqlx::query_as::<_, Project>(
        r#"
        SELECT id, name, project_key, description 
        FROM projects 
        WHERE id = ? 
        AND EXISTS (
            SELECT 1 FROM project_members 
            WHERE project_id = ? AND user_id = ?
        )
        "#,
    )
    .bind(project_id)
    .bind(project_id)
    .bind(user_id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    // Ak projekt existuje a používateľ má prístup, vrátime ho.
    // Inak vrátime chybu.
    match project {
        Some(p) => Ok(p),
        None => Err(ServerFnError::new("Access denied or project does not exist.")),
    }
}

// DTO pre člena projektu.
// Posiela sa na frontend napríklad do selectu pri priraďovaní tasku.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct ProjectMemberDto {
    pub id: i64,
    pub username: String,
}

// Serverová funkcia na načítanie členov konkrétneho projektu.
#[server]
pub async fn get_project_members(project_id: i64) -> Result<Vec<ProjectMemberDto>, ServerFnError> {
    // Zistíme aktuálne prihláseného používateľa.
    let user_id = get_current_user()
        .await?
        .ok_or_else(|| ServerFnError::new("Unauthorized"))?;

    // Získame databázový pool.
    let axum::Extension(pool) =
        leptos_axum::extract::<axum::Extension<sqlx::SqlitePool>>()
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

    // Najprv overíme, či je aktuálny používateľ členom projektu.
    // Ak nie je, nemal by vidieť zoznam členov.
    let is_member = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM project_members WHERE project_id = ? AND user_id = ?)",
    )
    .bind(project_id)
    .bind(user_id)
    .fetch_one(&pool)
    .await
    .unwrap_or(false);

    // Ak používateľ nie je členom projektu, vrátime Access denied.
    if !is_member {
        return Err(ServerFnError::new("Access denied"));
    }

    // Načítame všetkých členov projektu.
    let members = sqlx::query_as::<_, ProjectMemberDto>(
        r#"
        SELECT u.id, u.username 
        FROM users u
        JOIN project_members pm ON u.id = pm.user_id
        WHERE pm.project_id = ?
        "#,
    )
    .bind(project_id)
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(members)
}

// Serverová funkcia na načítanie projektov aktuálneho používateľa.
// Názov structu ProjectMemberDto je tu trochu mätúci,
// lebo sa používa aj na jednoduchý zoznam projektov.
#[server]
pub async fn get_my_projects() -> Result<Vec<ProjectMemberDto>, ServerFnError> {
    // Overíme aktuálneho používateľa.
    let user_id = get_current_user()
        .await?
        .ok_or_else(|| ServerFnError::new("Unauthorized"))?;

    // Získame databázový pool.
    let axum::Extension(pool) =
        leptos_axum::extract::<axum::Extension<sqlx::SqlitePool>>()
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

    // Načítame projekty, kde je aktuálny používateľ členom.
    // Pozor: query vyberá p.name, ale ProjectMemberDto má field username.
    // Ak to v projekte funguje, môže to byť riešené aliasom inde,
    // ale čistejšie by bolo použiť `p.name as username`.
    let projects = sqlx::query_as::<_, ProjectMemberDto>(
        r#"
        SELECT p.id, p.name 
        FROM projects p
        JOIN project_members pm ON p.id = pm.project_id
        WHERE pm.user_id = ?
        "#,
    )
    .bind(user_id)
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(projects)
}