# Issue Tracker

Fullstack aplikácia na správu úloh a projektov vytvorená v **Ruste**, **Leptose** a **SQLite**.

## Funkcie

### 🔐 Autentifikácia a Autorizácia
- Registrácia nových používateľov s bcrypt hashovaním hesiel
- Bezpečné prihlásenie s HTTP-only cookies
- Chránené cesty a prevencia IDOR útokov
- Rôlové prístupy (vlastník/člen)

### 📋 Správa Projektov
- Vytváranie a spravovanie projektov
- Pozývanie členov tímu
- Sledovanie štatistík projektov a analytiky tímu
- Prehliadanie všetkých projektov alebo filtrovaných podľa príslušnosti

### 🎯 Správa Úloh
- Kanban tabuľa s funkciou drag-and-drop
- Vytváranie, aktualizácia a priraďovanie úloh
- Viacero možností stavu (Zrobí sa, V progrese, Na kontrole, Hotovo)
- Modálne okno s úplnými informáciami o úlohe

### 💬 Spolupráca
- Vnorené komentáre s funkciou odpovedania
- Systém hlasov pre komentáre
- Vlákna diskusií v reálnom čase
- Podpora Markdownu

### ⏱️ Sledovanie Času
- Spustenie/zastavenie časovača pre úlohy
- Ručné zaznamenávanie času
- Prehliadanie histórie sledovania času
- Dashboard analytiky tímu s distribúciou času

### 📊 Analytika
- Štatistiky výkonu tímu
- Distribúcia času medzi členov tímu
- Vizuálne stĺpcové grafy
- Poznatky na úrovni projektu

### 🔔 Používateľské Rozhranie
- Systém upozornení (úspech/chyba/info)
- Responzívny dizajn pre počítač a mobilný telefón
- Čisté a moderné rozhranie
- Hladké animácie a prechody

---

## Tech Stack

### Frontend
- **Leptos** - Fullstack Rust framework
- **Leptos Router** - Klientske smerovania
- **Leptos Meta** - Správa meta tagov
- **CSS** - Vlastné štýly

### Backend
- **Leptos Server Functions** - Bezchybná komunikácia klient-server
- **Axum** - Web framework
- **SQLx** - Type-safe SQL dotazy
- **SQLite** - Databáza

### Bezpečnosť
- **Bcrypt** - Hešovanie hesiel
- **HTTP-only Cookies** - Správa relácií
- **IDOR Prevention** - Validácia prístupu k projektom

---

## Začíname

### Požiadavky
- **Rust** (1.70+) s Cargo
- **Node.js** (pre nástroje na zostavenie Leptos)
- **SQLite** 3

### Inštalácia

1. **Klonovať repozitár**
   ```bash
   git clone <adresa-repozitára>
   cd issue-tracker
   ```

2. **Nainštalovať závislosti**
   ```bash
   cargo install cargo-leptos
   ```

3. **Inicializovať databázu**
   ```bash
   # Vytvorenie SQLite databázy a spustenie migrácií
   sqlite3 tracker.db < schema.sql
   ```

4. **Naplniť testovacími dátami (voliteľné)**
   ```bash
   # Aplikácia automaticky naplní vzorové dáta pri prvom spustení
   # Používatelia: adam@test.cc, laco@test.cc, jana@test.cc, peter@test.cc
   # Heslo: heslo123
   ```

### Spustenie Aplikácie

**Vývojový režim:**
```bash
cargo leptos watch
```

**Build na produkciu:**
```bash
cargo leptos build --release
```

Aplikácia bude dostupná na `http://localhost:3000`

---

## Schéma Databázy

### Používatelia
- `id` - Primárny kľúč
- `username` - Zobrazované meno používateľa
- `email` - Jedinečný e-mail
- `password_hash` - Bcrypt hešované heslo

### Projekty
- `id` - Primárny kľúč
- `name` - Názov projektu
- `project_key` - Krátky identifikátor (napr. ESHOP)
- `description` - Popis projektu
- `owner_id` - Vlastník projektu (user_id)

### Členovia Projektu
- `project_id` - Cudzí kľúč na projekty
- `user_id` - Cudzí kľúč na používateľov
- `role` - 'owner' alebo 'member'

### Úlohy
- `id` - Primárny kľúč
- `project_id` - Cudzí kľúč na projekty
- `title` - Názov úlohy
- `description` - Popis úlohy
- `status` - Zrobí sa, V progrese, Na kontrole, Hotovo
- `assignee_id` - Priradený používateľ (nullable)

### Komentáre
- `id` - Primárny kľúč
- `task_id` - Cudzí kľúč na úlohy
- `user_id` - Cudzí kľúč na používateľov
- `parent_id` - Pre vnorené odpovede (nullable)
- `content` - Text komentára
- `upvotes` - Počet hlasov
- `created_at` - Časová značka

### Záznamy o Čase
- `id` - Primárny kľúč
- `task_id` - Cudzí kľúč na úlohy
- `user_id` - Cudzí kľúč na používateľov
- `duration_seconds` - Zaznamenávaný čas v sekundách
- `created_at` - Časová značka

---

## Funkcie Bezpečnosti

### Autentifikácia
- Heslá sú hešované pomocou bcrypt s DEFAULT_COST
- Relácie sú uložené v HTTP-only cookies (nie sú prístupné cez JavaScript)
- Auto-logout po 24 hodinách
- HTTPS sa odporúča pre produkciu

### Autorizácia
- **Prevencia IDOR** - Všetok prístup k úlohám/komentárom je overený voči príslušnosti v projekte
- **Kontrola Prístupu k Projektom** - Používatelia môžu prezerať/upravovať len projekty, v ktorých sú členmi
- **Rôlové Prístupy** - Operácie len pre vlastníka (zmazanie projektu, odstránenie člena)

### Validácia Dát
- SQL parameterované dotazy zabraňujú SQL injekcii
- Type-safe serverové funkcie pomocou Leptos
- Validácia vstupu na strane servera

---

## Testovacie účty

Predvolení testovací používatelia v databáze:

| E-mail | Heslo | Rola |
|--------|-------|------|
| adam@test.cc | heslo123 | Vlastník |
| laco@test.cc | heslo123 | Člen |
| jana@test.cc | heslo123 | Člen |
| peter@test.cc | heslo123 | Člen |

---

## Prehľad API

### Autentifikácia
- `register_user(username, email, password)` - Vytvorenie nového účtu
- `login_user(email, password)` - Prihlásenie
- `logout_user()` - Odhlásenie
- `get_current_user()` - Získanie ID autentifikovaného používateľa
- `get_user_profile()` - Získanie detailov používateľa

### Projekty
- `get_projects()` - Zoznam projektov používateľa
- `create_project(name, project_key, description, invited_users)` - Vytvorenie projektu
- `get_project(id)` - Získanie detailov projektu
- `get_project_members(id)` - Zoznam členov projektu
- `get_project_stats(id)` - Získanie analytických dát
- `get_my_projects()` - Projekty, ktorých je používateľ členom

### Úlohy
- `get_tasks(project_id)` - Zoznam úloh v projekte
- `create_task(project_id, title, description, status)` - Vytvorenie úlohy
- `get_task(id)` - Získanie detailov úlohy
- `update_task_status(id, status)` - Zmena stavu úlohy
- `assign_task(id, assignee_id)` - Priraďovanie úlohy používateľovi
- `get_my_issues()` - Úlohy priradené používateľovi

### Komentáre
- `get_comments(task_id)` - Zoznam komentárov k úlohe
- `add_comment(task_id, parent_id, content)` - Zverejnenie komentára
- `upvote_comment(id)` - Hlasovanie pre komentár

### Sledovanie Času
- `get_time_entries(task_id)` - Zoznam záznamov o čase
- `add_time_entry(task_id, duration_seconds)` - Zaznamenanie času

---

## Kvalita Kódu

### Najlepšie Praktiky
- **Type Safety** - Využíva Rustov systém typov a type-safe serverové funkcie Leptos
- **Spracovanie Chýb** - Správne typy `Result` s zmyslyplnými chybovými správami
- **Minimálne Komentáre** - Kód je samo-dokumentujúci sa s jasnými názvami konvencií
- **Konzistentný Štýl** - Dodržiava Rustove konvencie a najlepšie praktiky Leptos
- **Oddelenie Záležitostí** - Jasné hranice medzi frontend, backend a vrstvou dát

### Prehľad Súborov
- **19 Rust modulov** - Všetky organizované podľa funkčnosti
- **Bez externých API volaní** - Samostatná aplikácia
- **Type-safe databázové dotazy** - Použitie SQLx compile-time overenia
- **Fullstack Rust** - Type safety od databázy až po UI

---
## Výkon a optimalizace (Performance Considerations)

- **Lazy Loading (Odložené načítání)** - Komentáře a úlohy se načítavají dynamicky až na vyžiadanie.
- **Cachování** - Zdroje jsou pre zrýchlenie cachované na strane klienta.
- **Indexovanie databázy** - Databázové dopyty sú optimalizované pomocou správne nastavených indexov.
- **Responzívne UI** - Plynulé CSS prechody a animácie pre lepší používateľský zážitok.
- **Optimalizácia kódu** - Minimálna veľkosť výsledného bundlu vďaka architektúre frameworku Leptos.

---

## Budúce vylepšenia (Roadmapa)

- [ ] **Stránkovanie (Pagination)** - Efektívne spracovanie a zobrazovanie veľkých objemov dát.
- [ ] **Pokročilé filtrovanie** - Filtrovanie úloh podľa stavu, priradenej osoby a dátumu.
- [ ] **Vyhľadávanie** - Full-textové vyhľadávanie naprieč projektmi a úlohami.
- [ ] **Notifikácie** - E-mailové upozornenia a notifikácie priamo v aplikácii.
- [ ] **Označovanie používateľov** - Možnosť spomenúť kolegu cez @pouzivatel v komentároch.
- [ ] **Prílohy** - Nahrávanie súborov priamo k úlohám alebo komentárom.
- [ ] **Tmavý režim (Dark Mode)** - Prepínač vizuálnych tém.
- [ ] **Záznam aktivity** - Chronologický log zmien a aktivít v projekte.
- [ ] **API Dokumentácia** - Špecifikácia endpointov pomocou OpenAPI/Swagger.
- [ ] **Integračné testy** - Komplexná sada automatizovaných testov pre overenie stability.

---

## Riešenie problémov (Troubleshooting)

### Chyba pripojenia k databáze
- Uisti sa, že máš nainštalované SQLite: `sqlite3 --version`
- Skontroluj, či existuje súbor s databázou: `ls -la tracker.db`
- Over prístupové práva k databázovému súboru: `chmod 644 tracker.db`

### Port sa už používa
- Predvolený port je 3000. Zmeniť ho môžeš takto: `LEPTOS_ADDR=127.0.0.1:3001 cargo leptos watch`

### Chyba pri kompilácii (Build fails)
- Aktualizuj Rust na najnovšiu verziu: `rustup update`
- Vymaž cache a spusti build znova: `cargo clean && cargo leptos build`

---

## Prispievanie do projektu (Contributing)

Príspevky do kódu sú vítané! Postupuj prosím nasledovne:
1. Urob Fork tohto repozitára.
2. Vytvor si novú vetvu pre svoju funkciu (`git checkout -b feature/uzasna-funkcia`).
3. Urob commit svojich zmien (`git commit -m 'Pridanie úžasnej funkcie'`).
4. Pushni zmeny do svojej vetvy (`git push origin feature/uzasna-funkcia`).
5. Otvor Pull Request.

---

## Licencia

Tento projekt je licencovaný pod licenciou MIT – pre viac detailov si pozri súbor LICENSE.

---

## Autor

Vytvorené ako semestrálny projekt demonštrujúci full-stack vývoj webových aplikácií v jazyku Rust s využitím moderných *best practices*, dizajnu zameraného na bezpečnosť a čistej architektúry.

---

## Podpora

V prípade problémov, otázok alebo návrhov na zlepšenie prosím vytvor nové "Issue" v tomto repozitári.

**Príjemné trackovanie úloh! 🚀**