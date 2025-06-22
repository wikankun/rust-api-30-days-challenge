# 30-Day Rust API Challenge (One Day, One API)

Welcome! This challenge helps you learn Rust by building a new mini-API every day using Axum, one concept at a time.

---

## Week 1 – Core Rust + Axum Basics

- [x] **Day 1:** Hello World API (GET `/`)
- [x] **Day 2:** Path Parameters (GET `/hello/:name`)
- [x] **Day 3:** Extractor: Query Params (GET `/sum?a=1&b=2`)
- [ ] **Day 4:** Extractor: JSON Body (POST `/echo`)
- [ ] **Day 5:** Multiple REST Methods (GET, POST, PUT, DELETE `/item`)
- [ ] **Day 6:** Return Different Response Types (Plain Text, JSON, Custom Status)
- [ ] **Day 7:** Axum Router Nesting and Route Grouping

---

## Week 2 – Middleware, Error Handling, and Auth

- [ ] **Day 8:** Logging middleware (`tower-http`)
- [ ] **Day 9:** Enable CORS for Frontend Access
- [ ] **Day 10:** Rate Limiting (Basic Token Bucket or Counter)
- [ ] **Day 11:** Custom Error Types and Global Error Handler
- [ ] **Day 12:** JWT Authentication (Mock, Header Parsing)
- [ ] **Day 13:** Session With Cookies (or Local State)
- [ ] **Day 14:** Load Environment Variables (`dotenvy`)

---

## Week 3 – Storage & State

- [ ] **Day 15:** SQLite Integration with `sqlx`
- [ ] **Day 16:** PostgreSQL Example
- [ ] **Day 17:** Redis Visit Counter
- [ ] **Day 18:** File Upload API
- [ ] **Day 19:** Serve Static Files (`/static/logo.png`)
- [ ] **Day 20:** Shared Global State with `Arc<Mutex<T>>`
- [ ] **Day 21:** Current Timestamp API

---

## Week 4 – Tools, Testing, Deploy

- [ ] **Day 22:** JSON → CSV Converter API
- [ ] **Day 23:** Image Resize API
- [ ] **Day 24:** PDF Metadata Extractor
- [ ] **Day 25:** Async Background Task
- [ ] **Day 26:** Unit Test a Handler
- [ ] **Day 27:** Generate Swagger (OpenAPI)
- [ ] **Day 28:** Dockerize the API
- [ ] **Day 29:** Freestyle Day – Build your own
- [ ] **Day 30:** Deploy to Fly.io or Render idea!

---

## Crates to Explore

- `axum`, `tokio`, `tower`, `tower-http`
- `serde`, `serde_json`, `uuid`
- `sqlx`, `sea-orm`, `redis`
- `dotenvy`, `utoipa`

---

### Tips:
- Use `cargo new day_XX` to start fresh daily. <!-- Or use `cargo init --bin --vcs none` -->
- Use `cargo add <package>` to add package dependencies to Cargo.toml
- Run `cargo install cargo-watch` and use `cargo watch -x run` to live-reload during dev.
- Use `curl` or `httpie` to test quickly.
- Feel free to mix in your own ideas!

---

Happy hacking and welcome to the Rustacean life 🦀🔥
