// Importujeme základné veci z Leptosu.
// Tu používame hlavne #[server] a ServerFnError.
use leptos::prelude::*;

// Serde používame na serializáciu a deserializáciu dát.
// Je to potrebné, keď posielame structy medzi serverom a frontendom.
use serde::{Deserialize, Serialize};

// Funkcia na zistenie aktuálne prihláseného používateľa.
use crate::server::auth::get_current_user;

// Pomocná funkcia na kontrolu, či má používateľ prístup k projektu.
// Je dostupná iba na serveri, lebo pracuje priamo s databázou.
#[cfg(feature = "ssr")]
async fn check_project_access(
    pool: &sqlx::SqlitePool,
    user_id: i64,
    project_id: i64,
) -> Result<(), ServerFnError> {
    // Overíme, či existuje záznam v tabuľke project_members.
    // Teda či je daný používateľ členom daného projektu.
    let is_member = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM project_members WHERE project_id = ? AND user_id = ?)",
    )
    .bind(project_id)
    .bind(user_id)
    .fetch_one(pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    // Ak používateľ nie je členom projektu, vrátime chybu.
    if !is_member {
        return Err(ServerFnError::new(
            "Access denied: You are not a member of this project.",
        ));
    }

    // Ak kontrola prešla, funkcia skončí úspešne.
    Ok(())
}

// DTO pre task.
// DTO je jednoduchý dátový objekt, ktorý posielame zo servera na frontend.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct TaskDto {
    pub id: i64,
    pub project_id: i64,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub assignee_id: Option<i64>,
}

// Serverová funkcia na načítanie taskov konkrétneho projektu.
#[server]
pub async fn get_tasks(project_id: i64) -> Result<Vec<TaskDto>, ServerFnError> {
    // Najprv zistíme aktuálne prihláseného používateľa.
    // Ak používateľ nie je prihlásený, vrátime chybu.
    let user_id = get_current_user()
        .await?
        .ok_or_else(|| ServerFnError::new("Unauthorized access."))?;

    #[cfg(feature = "ssr")]
    {
        use axum::Extension;
        use sqlx::SqlitePool;

        // Získame databázový pool.
        let Extension(pool) = leptos_axum::extract::<Extension<SqlitePool>>()
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        // Overíme, či má používateľ prístup k projektu.
        check_project_access(&pool, user_id, project_id).await?;

        // Načítame všetky tasky patriace k danému projektu.
        // ORDER BY id DESC znamená, že najnovšie tasky budú hore.
        let tasks = sqlx::query_as::<_, TaskDto>(
            "SELECT * FROM tasks WHERE project_id = ? ORDER BY id DESC",
        )
        .bind(project_id)
        .fetch_all(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

        Ok(tasks)
    }

    // Klientsky fallback, aby sa kód vedel skompilovať aj mimo SSR.
    #[cfg(not(feature = "ssr"))]
    {
        let _ = project_id;
        Ok(vec![])
    }
}

// Serverová funkcia na vytvorenie nového tasku.
#[server]
pub async fn create_task(
    project_id: i64,
    title: String,
    description: String,
    status: String,
) -> Result<(), ServerFnError> {
    // Zistíme aktuálne prihláseného používateľa.
    let user_id = get_current_user()
        .await?
        .ok_or_else(|| ServerFnError::new("Unauthorized access."))?;

    #[cfg(feature = "ssr")]
    {
        use axum::Extension;
        use sqlx::SqlitePool;

        // Získame databázový pool.
        let Extension(pool) = leptos_axum::extract::<Extension<SqlitePool>>()
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        // Pred vytvorením tasku overíme, či používateľ patrí do projektu.
        check_project_access(&pool, user_id, project_id).await?;

        // Vložíme nový task do databázy.
        sqlx::query(
            "INSERT INTO tasks (project_id, title, description, status) VALUES (?, ?, ?, ?)",
        )
        .bind(project_id)
        .bind(title)
        .bind(description)
        .bind(status)
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

        Ok(())
    }

    // Klientsky fallback.
    #[cfg(not(feature = "ssr"))]
    {
        let _ = (project_id, title, description, status);
        Ok(())
    }
}

// Serverová funkcia na zmenu statusu tasku.
// Používa sa napríklad pri presúvaní tasku medzi Kanban stĺpcami.
#[server]
pub async fn update_task_status(task_id: i64, new_status: String) -> Result<(), ServerFnError> {
    // Zistíme aktuálne prihláseného používateľa.
    let user_id = get_current_user()
        .await?
        .ok_or_else(|| ServerFnError::new("Unauthorized access."))?;

    #[cfg(feature = "ssr")]
    {
        use axum::Extension;
        use sqlx::SqlitePool;

        // Získame databázový pool.
        let Extension(pool) = leptos_axum::extract::<Extension<SqlitePool>>()
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        // Najprv si zistíme, ku ktorému projektu task patrí.
        let project_id = sqlx::query_scalar::<_, i64>("SELECT project_id FROM tasks WHERE id = ?")
            .bind(task_id)
            .fetch_optional(&pool)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?
            .ok_or_else(|| ServerFnError::new("Task does not exist"))?;

        // Overíme, či používateľ má prístup k projektu daného tasku.
        check_project_access(&pool, user_id, project_id).await?;

        // Aktualizujeme status tasku v databáze.
        sqlx::query("UPDATE tasks SET status = ? WHERE id = ?")
            .bind(new_status)
            .bind(task_id)
            .execute(&pool)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        Ok(())
    }

    // Klientsky fallback.
    #[cfg(not(feature = "ssr"))]
    {
        let _ = (task_id, new_status);
        Ok(())
    }
}

// DTO pre komentár.
// Používa sa pri načítaní komentárov k tasku.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct CommentDto {
    pub id: i64,
    pub parent_id: Option<i64>,
    pub author_name: String,
    pub content: String,
    pub upvotes: i32,
    pub created_at: String,
}

// Serverová funkcia na načítanie komentárov ku konkrétnemu tasku.
#[server]
pub async fn get_comments(task_id: i64) -> Result<Vec<CommentDto>, ServerFnError> {
    // Najprv overíme, že používateľ je prihlásený.
    let user_id = get_current_user()
        .await?
        .ok_or_else(|| ServerFnError::new("Unauthorized"))?;

    #[cfg(feature = "ssr")]
    {
        use axum::Extension;
        use sqlx::SqlitePool;

        // Získame databázový pool.
        let Extension(pool) = leptos_axum::extract::<Extension<SqlitePool>>()
            .await
            .unwrap();

        // Podľa task_id zistíme project_id.
        // Potrebujeme to na kontrolu, či používateľ má prístup k projektu.
        let project_id = sqlx::query_scalar::<_, i64>("SELECT project_id FROM tasks WHERE id = ?")
            .bind(task_id)
            .fetch_one(&pool)
            .await
            .map_err(|_| ServerFnError::new("Unauthorized"))?;

        // Overíme prístup k projektu.
        check_project_access(&pool, user_id, project_id).await?;

        // Načítame komentáre k tasku.
        // JOIN na users používame preto, aby sme vedeli zobraziť meno autora.
        let comments = sqlx::query_as::<_, CommentDto>(
            r#"
            SELECT c.id, c.parent_id, u.username as author_name, c.content, c.upvotes, c.created_at
            FROM comments c
            JOIN users u ON c.user_id = u.id
            WHERE c.task_id = ?
            ORDER BY c.created_at ASC
            "#,
        )
        .bind(task_id)
        .fetch_all(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

        Ok(comments)
    }

    // Klientsky fallback.
    #[cfg(not(feature = "ssr"))]
    {
        let _ = task_id;
        Ok(vec![])
    }
}

// Serverová funkcia na pridanie komentára k tasku.
// parent_id je Option, lebo komentár môže byť hlavný komentár alebo odpoveď.
#[server]
pub async fn add_comment(
    task_id: i64,
    parent_id: Option<i64>,
    content: String,
) -> Result<(), ServerFnError> {
    // Overíme prihláseného používateľa.
    let user_id = get_current_user()
        .await?
        .ok_or_else(|| ServerFnError::new("Unauthorized"))?;

    #[cfg(feature = "ssr")]
    {
        use axum::Extension;
        use sqlx::SqlitePool;

        // Získame databázový pool.
        let Extension(pool) = leptos_axum::extract::<Extension<SqlitePool>>()
            .await
            .unwrap();

        // Zistíme, ku ktorému projektu task patrí.
        let project_id = sqlx::query_scalar::<_, i64>("SELECT project_id FROM tasks WHERE id = ?")
            .bind(task_id)
            .fetch_one(&pool)
            .await
            .map_err(|_| ServerFnError::new("Unauthorized"))?;

        // Overíme, že používateľ má k projektu prístup.
        check_project_access(&pool, user_id, project_id).await?;

        // Vložíme nový komentár do databázy.
        // Ak je parent_id None, ide o hlavný komentár.
        // Ak je parent_id Some(id), ide o odpoveď na iný komentár.
        sqlx::query(
            "INSERT INTO comments (task_id, user_id, parent_id, content) VALUES (?, ?, ?, ?)",
        )
        .bind(task_id)
        .bind(user_id)
        .bind(parent_id)
        .bind(content)
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

        Ok(())
    }

    // Klientsky fallback.
    #[cfg(not(feature = "ssr"))]
    {
        let _ = (task_id, parent_id, content);
        Ok(())
    }
}

// Serverová funkcia na upvote komentára.
#[server]
pub async fn upvote_comment(comment_id: i64) -> Result<(), ServerFnError> {
    // Overíme, že používateľ je prihlásený.
    let user_id = get_current_user()
        .await?
        .ok_or_else(|| ServerFnError::new("Unauthorized"))?;

    #[cfg(feature = "ssr")]
    {
        use axum::Extension;
        use sqlx::SqlitePool;

        // Získame databázový pool.
        let Extension(pool) = leptos_axum::extract::<Extension<SqlitePool>>()
            .await
            .unwrap();

        // Podľa komentára zistíme project_id.
        // Ideme cez comments -> tasks -> project_id.
        let project_id = sqlx::query_scalar::<_, i64>(
            "SELECT t.project_id FROM tasks t JOIN comments c ON c.task_id = t.id WHERE c.id = ?",
        )
        .bind(comment_id)
        .fetch_one(&pool)
        .await
        .map_err(|_| ServerFnError::new("Unauthorized"))?;

        // Overíme, že používateľ patrí do projektu.
        check_project_access(&pool, user_id, project_id).await?;

        // Zvýšime počet upvotov komentára o 1.
        sqlx::query("UPDATE comments SET upvotes = upvotes + 1 WHERE id = ?")
            .bind(comment_id)
            .execute(&pool)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        Ok(())
    }

    // Klientsky fallback.
    #[cfg(not(feature = "ssr"))]
    {
        let _ = comment_id;
        Ok(())
    }
}

// DTO pre jeden záznam odpracovaného času.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct TimeEntryDto {
    pub id: i64,
    pub user_name: String,
    pub duration_seconds: i64,
    pub created_at: String,
}

// Serverová funkcia na načítanie časových záznamov pre konkrétny task.
#[server]
pub async fn get_time_entries(task_id: i64) -> Result<Vec<TimeEntryDto>, ServerFnError> {
    // Overíme prihláseného používateľa.
    let user_id = get_current_user()
        .await?
        .ok_or_else(|| ServerFnError::new("Unauthorized"))?;

    #[cfg(feature = "ssr")]
    {
        use axum::Extension;
        use sqlx::SqlitePool;

        // Získame databázový pool.
        let Extension(pool) = leptos_axum::extract::<Extension<SqlitePool>>()
            .await
            .unwrap();

        // Zistíme projekt podľa tasku.
        let project_id = sqlx::query_scalar::<_, i64>("SELECT project_id FROM tasks WHERE id = ?")
            .bind(task_id)
            .fetch_one(&pool)
            .await
            .map_err(|_| ServerFnError::new("Unauthorized"))?;

        // Overíme prístup k projektu.
        check_project_access(&pool, user_id, project_id).await?;

        // Načítame časové záznamy.
        // JOIN na users používame kvôli menu používateľa.
        let entries = sqlx::query_as::<_, TimeEntryDto>(
            r#"
            SELECT t.id, u.username as user_name, t.duration_seconds, t.created_at
            FROM time_entries t
            JOIN users u ON t.user_id = u.id
            WHERE t.task_id = ?
            ORDER BY t.created_at DESC
            "#,
        )
        .bind(task_id)
        .fetch_all(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

        Ok(entries)
    }

    // Klientsky fallback.
    #[cfg(not(feature = "ssr"))]
    {
        let _ = task_id;
        Ok(vec![])
    }
}

// Serverová funkcia na uloženie nového časového záznamu.
// Používa sa pri zastavení timeru na frontende.
#[server]
pub async fn add_time_entry(task_id: i64, duration_seconds: i64) -> Result<(), ServerFnError> {
    // Overíme aktuálneho používateľa.
    let user_id = get_current_user()
        .await?
        .ok_or_else(|| ServerFnError::new("Unauthorized"))?;

    #[cfg(feature = "ssr")]
    {
        use axum::Extension;
        use sqlx::SqlitePool;

        // Získame databázový pool.
        let Extension(pool) = leptos_axum::extract::<Extension<SqlitePool>>()
            .await
            .unwrap();

        // Zistíme projekt podľa tasku.
        let project_id = sqlx::query_scalar::<_, i64>("SELECT project_id FROM tasks WHERE id = ?")
            .bind(task_id)
            .fetch_one(&pool)
            .await
            .map_err(|_| ServerFnError::new("Unauthorized"))?;

        // Overíme, že používateľ môže zapisovať čas k tomuto tasku.
        check_project_access(&pool, user_id, project_id).await?;

        // Vložíme nový time entry do databázy.
        // user_id hovorí, kto čas zapísal.
        sqlx::query(
            "INSERT INTO time_entries (task_id, user_id, duration_seconds) VALUES (?, ?, ?)",
        )
        .bind(task_id)
        .bind(user_id)
        .bind(duration_seconds)
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

        Ok(())
    }

    // Klientsky fallback.
    #[cfg(not(feature = "ssr"))]
    {
        let _ = (task_id, duration_seconds);
        Ok(())
    }
}

// Serverová funkcia na priradenie tasku používateľovi.
// assignee_id je Option, lebo task môže byť aj nepriradený.
#[server]
pub async fn assign_task(task_id: i64, assignee_id: Option<i64>) -> Result<(), ServerFnError> {
    // Overíme, že používateľ je prihlásený.
    // Tu sa user_id ďalej nepoužíva, iba sa kontroluje prihlásenie.
    get_current_user()
        .await?
        .ok_or_else(|| ServerFnError::new("Unauthorized"))?;

    // Získame databázový pool.
    let axum::Extension(pool) =
        leptos_axum::extract::<axum::Extension<sqlx::SqlitePool>>()
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

    // Aktualizujeme assignee_id pri danom tasku.
    // Ak je assignee_id None, task zostane nepriradený.
    sqlx::query("UPDATE tasks SET assignee_id = ? WHERE id = ?")
        .bind(assignee_id)
        .bind(task_id)
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(())
}

// Serverová funkcia na načítanie taskov, ktoré sú priradené aktuálnemu používateľovi.
#[server]
pub async fn get_my_issues() -> Result<Vec<TaskDto>, ServerFnError> {
    // Zistíme aktuálne prihláseného používateľa.
    let user_id = get_current_user()
        .await?
        .ok_or_else(|| ServerFnError::new("Unauthorized access."))?;

    // Získame databázový pool.
    let axum::Extension(pool) =
        leptos_axum::extract::<axum::Extension<sqlx::SqlitePool>>()
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

    // Načítame tasky, kde assignee_id je aktuálny používateľ.
    let tasks = sqlx::query_as::<_, TaskDto>(
        r#"
        SELECT id, project_id, title, description, status, assignee_id 
        FROM tasks 
        WHERE assignee_id = ?
        ORDER BY id DESC
        "#,
    )
    .bind(user_id)
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(tasks)
}

// Serverová funkcia na načítanie detailu jedného tasku.
#[server]
pub async fn get_task(task_id: i64) -> Result<TaskDto, ServerFnError> {
    // Získame databázový pool.
    let axum::Extension(pool) =
        leptos_axum::extract::<axum::Extension<sqlx::SqlitePool>>()
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

    // Načítame konkrétny task podľa jeho ID.
    let task = sqlx::query_as::<_, TaskDto>(
        "SELECT id, project_id, title, description, status, assignee_id FROM tasks WHERE id = ?",
    )
    .bind(task_id)
    .fetch_one(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(task)
}