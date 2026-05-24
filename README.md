# Issue Tracker

Fullstack aplikácia na správu projektov, úloh a tímovej spolupráce vytvorená v **Ruste** pomocou **Leptos**, **Axum**, **SQLx** a **SQLite**.

Projekt slúži ako semestrálna ukážka fullstack vývoja v Ruste. Obsahuje registráciu a prihlásenie používateľov, správu projektov, Kanban tabuľu, komentáre, sledovanie času a jednoduchý analytický dashboard.

---

## Funkcie

### 🔐 Autentifikácia a autorizácia
- Registrácia používateľov
- Prihlásenie pomocou bcrypt hashovania hesiel
- Session-based autentifikácia cez náhodné `session_id`
- `session_id` je uložené v HTTP-only cookie
- Session sa overuje voči tabuľke `sessions` v databáze
- Chránené stránky pomocou `ProtectedRoute`
- Kontrola prístupu k projektom cez členstvo v `project_members`

### 📋 Správa projektov
- Vytváranie projektov
- Pozývanie členov tímu pri vytvorení projektu
- Zobrazenie projektov, ktorých je používateľ členom
- Prechod na Kanban board konkrétneho projektu
- Zobrazenie členov projektu pri priraďovaní úloh

### 🎯 Správa úloh
- Vytváranie úloh v projekte
- Kanban tabuľa so stavmi `Todo`, `InProgress`, `InReview`, `Done`
- Drag-and-drop presúvanie úloh medzi stĺpcami
- Priraďovanie úloh členom projektu
- Detail úlohy v modálnom okne

### 💬 Komentáre
- Komentáre k úlohám
- Odpovede na komentáre cez `parent_id`
- Vnorené diskusné vlákna
- Upvote komentárov
- Automatické opätovné načítanie diskusie po pridaní komentára alebo upvote

### ⏱️ Sledovanie času
- Spustenie a zastavenie časovača pri úlohe
- Uloženie odpracovaného času do databázy
- Zobrazenie histórie časových záznamov
- Výpočet celkového času na úlohe

### 📊 Dashboard a analytika
- Výber projektu
- Celkový odpracovaný čas
- Počet aktívnych členov
- Priemerný čas na člena
- Top contributor
- Jednoduchý stĺpcový graf odpracovaného času podľa členov

### 🔔 Používateľské rozhranie
- Toast notifikácie pre úspech, chybu a informácie
- Responzívny sidebar
- Modálne okná pre vytvorenie projektu, vytvorenie úlohy a detail úlohy
- Jednoduchý moderný dizajn

---

## Tech Stack

### Frontend
- **Leptos** – fullstack Rust framework
- **Leptos Router** – routovanie stránok
- **Leptos Meta** – meta tagy, titulok a štýly
- **SCSS/CSS** – vlastné štýly

### Backend
- **Axum** – HTTP server
- **Leptos Server Functions** – komunikácia medzi frontendom a serverom
- **SQLx** – práca s databázou
- **SQLite** – lokálna databáza

### Bezpečnosť
- **bcrypt** – hashovanie hesiel
- **HTTP-only cookies** – ochrana session cookie pred JavaScriptom
- **Session tabuľka** – server overuje `session_id` voči databáze
- **Parameterized SQL queries** – ochrana pred SQL injection
- **Access control** – serverové kontroly členstva v projekte pri viacerých operáciách

---

## Požiadavky

- Rust a Cargo
- `cargo-leptos`
- SQLite

Inštalácia `cargo-leptos`:

```bash
cargo install cargo-leptos
```

---

## Nastavenie prostredia

V koreňovom priečinku projektu vytvor súbor `.env`:

```env
DATABASE_URL=sqlite:issue_tracker.db?mode=rwc
SEED_DB=1
```

Význam premenných:

- `DATABASE_URL` určuje SQLite databázu, ktorú aplikácia používa.
- `SEED_DB=1` zapne vloženie testovacích dát pri štarte aplikácie.

Po prvom úspešnom naplnení databázy môžeš `SEED_DB=1` zo súboru `.env` odstrániť alebo zakomentovať, aby sa seed nespúšťal pri každom štarte.

---

## Spustenie aplikácie

Vývojový režim:

```bash
cargo leptos watch
```

Aplikácia bude dostupná na:

```text
http://127.0.0.1:3000
```

Produkčný build:

```bash
cargo leptos build --release
```

---

## Databáza a migrácie

Aplikácia používa SQLx migrácie v priečinku:

```text
migrations/
```

Pri štarte servera sa automaticky spustí:

```rust
sqlx::migrate!("./migrations")
```

To vytvorí potrebné tabuľky, napríklad:

- `users`
- `projects`
- `project_members`
- `tasks`
- `comments`
- `time_entries`
- `sessions`

Používaná SQLite databáza podľa `.env`:

```text
issue_tracker.db
```

Pri problémoch je možné overiť tabuľky cez Python:

```bash
python3 - <<'PY'
import sqlite3

con = sqlite3.connect("issue_tracker.db")

for row in con.execute("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name"):
    print(row[0])
PY
```

---

## Testovacie účty

Seed vytvára tieto testovacie účty. Všetky majú rovnaké heslo:

```text
password123
```

| ID | Meno | E-mail | Heslo |
|----|------|--------|-------|
| 1 | Adam Smith | adam@test.cc | password123 |
| 2 | Laco Logic | laco@test.cc | password123 |
| 3 | Jana Code | jana@test.cc | password123 |
| 4 | Peter Project | peter@test.cc | password123 |
| 5 | Mia Design | mia@test.cc | password123 |
| 6 | Tomas Tester | tomas@test.cc | password123 |
| 7 | Eva Security | eva@test.cc | password123 |
| 8 | Roman Rust | roman@test.cc | password123 |

Odporúčaný účet na testovanie:

```text
adam@test.cc
password123
```

---

## Schéma databázy

### `users`
- `id`
- `username`
- `email`
- `password_hash`
- `created_at`

### `projects`
- `id`
- `name`
- `project_key`
- `description`
- `owner_id`
- `created_at`

### `project_members`
- `project_id`
- `user_id`
- `role`
- `joined_at`

### `tasks`
- `id`
- `project_id`
- `title`
- `description`
- `status`
- `assignee_id`
- `created_at`

### `comments`
- `id`
- `task_id`
- `user_id`
- `parent_id`
- `content`
- `upvotes`
- `created_at`

### `time_entries`
- `id`
- `task_id`
- `user_id`
- `duration_seconds`
- `created_at`

### `sessions`
- `id`
- `user_id`
- `expires_at`
- `created_at`

---

## Prehľad serverových funkcií

### Autentifikácia
- `register_user(username, email, password)`
- `login_user(email, password)`
- `logout_user()`
- `get_current_user()`
- `get_user_profile()`
- `search_users(query)`

### Projekty
- `get_projects()`
- `create_project(name, project_key, description, invited_users_str)`
- `get_project(project_id)`
- `get_project_members(project_id)`
- `get_project_stats(project_id)`
- `get_my_projects()`

### Úlohy
- `get_tasks(project_id)`
- `create_task(project_id, title, description, status)`
- `get_task(task_id)`
- `update_task_status(task_id, new_status)`
- `assign_task(task_id, assignee_id)`
- `get_my_issues()`

### Komentáre
- `get_comments(task_id)`
- `add_comment(task_id, parent_id, content)`
- `upvote_comment(comment_id)`

### Sledovanie času
- `get_time_entries(task_id)`
- `add_time_entry(task_id, duration_seconds)`

---

## Bezpečnostné riešenia

### Hashovanie hesiel
Heslá sa neukladajú v čistom texte. Pri registrácii sa heslo zahashuje pomocou bcrypt a do databázy sa uloží iba `password_hash`.

### Session-based login
Po úspešnom prihlásení server vygeneruje náhodné `session_id`. Do cookie sa uloží iba toto `session_id`, nie `user_id`.

Server potom pri každej chránenej operácii vyhľadá session v tabuľke `sessions` a zistí, ktorému používateľovi patrí.

### Ochrana pred jednoduchou zmenou identity
Používateľ si síce vie lokálne prepísať cookie, ale ak si vymyslí neexistujúce `session_id`, server ho v databáze nenájde a používateľ nebude autentifikovaný.

### Kontrola prístupu
Pri práci s projektmi a úlohami sa na serveri overuje, či je používateľ členom príslušného projektu.

---

## Známe obmedzenia

Projekt je školská/semestrálna aplikácia, preto niektoré časti nie sú riešené produkčne:

- Upvote komentárov zatiaľ nekontroluje, či jeden používateľ hlasoval iba raz.
- Ukladanie popisu úlohy v detail modale je pripravené v UI, ale vyžaduje doplnenie serverovej funkcie.
- Status úlohy je uložený ako textový `String`; v produkcii by bolo vhodnejšie použiť enum alebo serverovú validáciu.
- Niektoré prístupové kontroly je možné ďalej sprísniť, napríklad pri detaile úlohy a priraďovaní úlohy.
- Aplikácia je určená hlavne na lokálne spustenie a prezentáciu.

---

## Roadmapa

- [ ] Validácia statusu úlohy na serveri
- [ ] Kontrola jedného upvotu na používateľa
- [ ] Reálne ukladanie popisu úlohy
- [ ] Rozšírené filtrovanie úloh
- [ ] Vyhľadávanie v projektoch a úlohách
- [ ] Notifikácie
- [ ] Prílohy k úlohám
- [ ] Audit log zmien
- [ ] Testy serverových funkcií

---

## Riešenie problémov

### `no such table: sessions`
Skontroluj, či prebehli migrácie a či existuje tabuľka `sessions`.

```bash
python3 - <<'PY'
import sqlite3
con = sqlite3.connect("issue_tracker.db")
print(con.execute("SELECT name FROM sqlite_master WHERE type='table' AND name='sessions'").fetchall())
PY
```

Ak tabuľka neexistuje, skontroluj priečinok `migrations/`, reštartuj server a podľa potreby použi:

```bash
cargo clean
cargo leptos watch
```

### Problém s cookies
Po zmene autentifikácie je vhodné vymazať staré cookies v prehliadači a prihlásiť sa znova.

### Port 3000 sa už používa
Ukonči existujúci proces alebo zmeň port v konfigurácii Leptos.

---

## Autor

Vytvorené ako semestrálny projekt demonštrujúci fullstack vývoj webovej aplikácie v jazyku Rust.

---

## Licencia

Projekt je určený na študijné účely.
