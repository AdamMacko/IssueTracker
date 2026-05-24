use sqlx::SqlitePool;
use bcrypt::{hash, DEFAULT_COST};

pub async fn seed_database(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    let password_hash = hash("password123", DEFAULT_COST).unwrap();

    // Testovací používatelia.
    // Všetci majú rovnaké heslo: password123
    let users = vec![
        (1, "Adam Smith", "adam@test.cc"),
        (2, "Laco Logic", "laco@test.cc"),
        (3, "Jana Code", "jana@test.cc"),
        (4, "Peter Project", "peter@test.cc"),
        (5, "Mia Design", "mia@test.cc"),
        (6, "Tomas Tester", "tomas@test.cc"),
        (7, "Eva Security", "eva@test.cc"),
        (8, "Roman Rust", "roman@test.cc"),
    ];

    for (id, name, email) in users {
        sqlx::query(
            "INSERT OR IGNORE INTO users (id, username, email, password_hash) VALUES (?, ?, ?, ?)",
        )
        .bind(id)
        .bind(name)
        .bind(email)
        .bind(&password_hash)
        .execute(pool)
        .await?;
    }

    // Testovacie projekty.
    let projects = vec![
        (
            1,
            "E-shop Mobile App",
            "ESHOP",
            "Complete redesign of the mobile shopping application.",
            1,
        ),
        (
            2,
            "Nuclear Audit System",
            "NUKE",
            "Internal system for nuclear regulatory authority.",
            1,
        ),
        (
            3,
            "School Cybersecurity Portal",
            "SCHOOL",
            "Portal for reporting and tracking cybersecurity incidents at schools.",
            3,
        ),
        (
            4,
            "Fitness Progress Tracker",
            "FIT",
            "Application for tracking workouts, progress, and training plans.",
            8,
        ),
    ];

    for (id, name, key, desc, owner_id) in projects {
        sqlx::query(
            "INSERT OR IGNORE INTO projects (id, name, project_key, description, owner_id) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(id)
        .bind(name)
        .bind(key)
        .bind(desc)
        .bind(owner_id)
        .execute(pool)
        .await?;
    }

    // Členovia projektov.
    let memberships = vec![
        // E-shop
        (1, 1, "owner"),
        (1, 2, "member"),
        (1, 3, "member"),
        (1, 5, "member"),
        (1, 6, "member"),

        // Nuclear Audit System
        (2, 1, "owner"),
        (2, 4, "member"),
        (2, 7, "member"),
        (2, 8, "member"),

        // School Cybersecurity Portal
        (3, 3, "owner"),
        (3, 1, "member"),
        (3, 6, "member"),
        (3, 7, "member"),

        // Fitness Progress Tracker
        (4, 8, "owner"),
        (4, 2, "member"),
        (4, 5, "member"),
        (4, 6, "member"),
    ];

    for (project_id, user_id, role) in memberships {
        sqlx::query(
            "INSERT OR IGNORE INTO project_members (project_id, user_id, role) VALUES (?, ?, ?)",
        )
        .bind(project_id)
        .bind(user_id)
        .bind(role)
        .execute(pool)
        .await?;
    }

    // Tasky naprieč projektmi.
    let tasks = vec![
        // Project 1 - E-shop Mobile App
        (1, 1, "Setup Leptos project", "Create base Leptos structure and routing.", "Done", Some(1)),
        (2, 1, "Auth implementation", "Login, registration and protected routes.", "Done", Some(1)),
        (3, 1, "Database migration", "Prepare SQLite schema and migrations.", "Done", Some(2)),
        (4, 1, "Frontend styling", "SCSS layout, responsive design and dashboard UI.", "InReview", Some(5)),
        (5, 1, "Product detail screen", "Create layout for product detail and image gallery.", "InProgress", Some(3)),
        (6, 1, "Cart bug fix", "Fix issue where quantity is not updated correctly.", "Todo", Some(6)),
        (7, 1, "Checkout validation", "Add validation for address and payment form.", "Todo", None),

        // Project 2 - Nuclear Audit System
        (8, 2, "Security audit", "Check for IDOR vulnerabilities and weak session handling.", "InProgress", Some(7)),
        (9, 2, "Role based access", "Restrict sensitive modules to authorized users only.", "Todo", Some(4)),
        (10, 2, "Audit log screen", "Display audit logs with filters and export option.", "InReview", Some(8)),
        (11, 2, "Incident report form", "Create form for reporting technical incidents.", "Done", Some(1)),
        (12, 2, "Session hardening", "Replace user_id cookie with database backed sessions.", "Done", Some(7)),

        // Project 3 - School Cybersecurity Portal
        (13, 3, "Risk register", "Create list of identified risks for school environment.", "Done", Some(3)),
        (14, 3, "Asset inventory", "Prepare primary and supporting asset inventory.", "InProgress", Some(1)),
        (15, 3, "Security measures table", "Map risks to technical and organizational measures.", "Todo", Some(7)),
        (16, 3, "Incident categories", "Define categories for phishing, malware and data leakage.", "InReview", Some(6)),
        (17, 3, "Teacher reporting flow", "Design simple workflow for teachers to report issues.", "Todo", None),

        // Project 4 - Fitness Progress Tracker
        (18, 4, "Workout model", "Design database model for workouts and exercises.", "Done", Some(8)),
        (19, 4, "Timer component", "Create timer for workout duration tracking.", "InProgress", Some(2)),
        (20, 4, "Progress charts", "Add charts for strength and endurance progress.", "Todo", Some(5)),
        (21, 4, "Mobile layout", "Improve layout for small screens.", "InReview", Some(6)),
        (22, 4, "Training notes", "Allow users to add notes to workout sessions.", "Todo", None),
    ];

    for (id, project_id, title, desc, status, assignee_id) in tasks {
        sqlx::query(
            "INSERT OR IGNORE INTO tasks (id, project_id, title, description, status, assignee_id) VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind(id)
        .bind(project_id)
        .bind(title)
        .bind(desc)
        .bind(status)
        .bind(assignee_id)
        .execute(pool)
        .await?;
    }

    // Time entries.
    // Tu používame pevné ID, aby sa pri opakovanom seede neduplikovali záznamy.
    let time_entries = vec![
        (1, 1, 1, 3600),
        (2, 1, 2, 1800),
        (3, 2, 1, 7200),
        (4, 3, 2, 5400),
        (5, 4, 5, 4200),
        (6, 5, 3, 6900),
        (7, 6, 6, 2400),
        (8, 8, 7, 8100),
        (9, 10, 8, 3600),
        (10, 11, 1, 4500),
        (11, 12, 7, 5200),
        (12, 13, 3, 6000),
        (13, 14, 1, 9300),
        (14, 15, 7, 2700),
        (15, 16, 6, 3900),
        (16, 18, 8, 7200),
        (17, 19, 2, 4800),
        (18, 20, 5, 3000),
        (19, 21, 6, 5100),
        (20, 4, 1, 1800),
        (21, 8, 4, 3600),
        (22, 14, 7, 2400),
    ];

    for (id, task_id, user_id, duration_seconds) in time_entries {
        sqlx::query(
            "INSERT OR IGNORE INTO time_entries (id, task_id, user_id, duration_seconds) VALUES (?, ?, ?, ?)",
        )
        .bind(id)
        .bind(task_id)
        .bind(user_id)
        .bind(duration_seconds)
        .execute(pool)
        .await?;
    }

    // Komentáre a odpovede.
    // parent_id = None znamená hlavný komentár.
    // parent_id = Some(id) znamená odpoveď na existujúci komentár.
    let comments: Vec<(i64, i64, i64, Option<i64>, &str, i32)> = vec![
        (
            1,
            2,
            2,
            None,
            "Adam, take a look at those cookies, they don't look right.",
            5,
        ),
        (
            2,
            2,
            1,
            Some(1),
            "Good catch. I replaced the user_id cookie with session_id.",
            4,
        ),
        (
            3,
            2,
            7,
            Some(2),
            "This is much safer. The server now verifies the session in the database.",
            6,
        ),
        (
            4,
            4,
            5,
            None,
            "I updated the dashboard colors and spacing. Please check the mobile view.",
            3,
        ),
        (
            5,
            4,
            6,
            Some(4),
            "Mobile looks better now, but the sidebar still needs small adjustments.",
            2,
        ),
        (
            6,
            8,
            7,
            None,
            "Found one more access control issue in task detail loading.",
            7,
        ),
        (
            7,
            8,
            1,
            Some(6),
            "I will add project membership validation to get_task as well.",
            3,
        ),
        (
            8,
            10,
            8,
            None,
            "Audit log filtering is mostly done, export still needs testing.",
            2,
        ),
        (
            9,
            13,
            3,
            None,
            "Risk register is ready for review. I added phishing and ransomware scenarios.",
            4,
        ),
        (
            10,
            14,
            1,
            None,
            "Asset inventory now separates primary and supporting assets.",
            5,
        ),
        (
            11,
            14,
            7,
            Some(10),
            "Nice. We should also mention backups and access management as supporting assets.",
            3,
        ),
        (
            12,
            16,
            6,
            None,
            "Incident categories are implemented. Can someone verify naming?",
            1,
        ),
        (
            13,
            19,
            2,
            None,
            "Workout timer works, but I want to reuse the same pattern as task time tracking.",
            2,
        ),
        (
            14,
            21,
            5,
            None,
            "Mobile layout is almost done. The cards need a little more padding.",
            2,
        ),
        (
            15,
            21,
            6,
            Some(14),
            "I tested it on a narrow screen and it is readable now.",
            1,
        ),
    ];

    for (id, task_id, user_id, parent_id, content, upvotes) in comments {
        sqlx::query(
            "INSERT OR IGNORE INTO comments (id, task_id, user_id, parent_id, content, upvotes) VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind(id)
        .bind(task_id)
        .bind(user_id)
        .bind(parent_id)
        .bind(content)
        .bind(upvotes)
        .execute(pool)
        .await?;
    }

    println!("Database seeded with expanded test data!");
    Ok(())
}