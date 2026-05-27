# OTP Authentication Service

This project is a Rust-based backend service built with `axum` and `sqlx`. It handles user authentication by validating a user against a PostgreSQL database and sending a One-Time Password (OTP) via the Twilio API using SMS.

## Tech Stack

- **Framework:** Rust / Axum
- **Database:** PostgreSQL (Run via Docker Compose) / `sqlx`
- **External API:** Twilio (Verify API)
- **HTTP Client:** `reqwest`

## Project Setup overview

1. **Database:** Spun up a Postgres instance using `docker-compose.yml`. Configured the connection in Axum using `sqlx::PgPool`.
2. **Twilio Integration:** Created a `TwilioService` that uses `reqwest` to make HTTP POST requests to Twilio's API to send and verify OTP codes.
3. **Routing:** Built `/signin` and `/verify` routes.
4. **State Management:** Injected the database pool (`State`) and external service (`Extension`/`State`) into the Axum router to reuse resources safely.

---

## 📝 Learning Notes & Q&A

### 1. `Extension` vs `State` in Axum

**Q: What is `Extension<TwilioService>` doing, and how is it different from `State`?**

- **Extension:** Acts as a hidden "storage box" in the HTTP request. It checks types at **runtime**. If you forget to provide it via `.layer()`, your app compiles but crashes with a 500 error when the route is hit.
- **State:** The modern, recommended way to pass dependencies (like a DB Pool). It checks types at **compile time**. If you forget `.with_state()`, the code won't even compile.
- **How it works:** Added via `.layer()` (middleware) at the start of the request, and read via an extractor (`service: Extension<...>`) at the end inside the handler.

### 2. Hardcoding vs. Dependency Injection & Middleware

**Q: Could I just hardcode the Twilio service creation directly in the handler? What really is middleware?**

- **Hardcoding vs State:** If you instantiate `TwilioService` in the handler, it creates a new HTTP client and reads environment variables on _every single request_. Passing it via `State` or `Extension` allows you to create the HTTP client once and reuse it, saving memory and time.
- **Middleware:** It wraps handlers like an onion. It allows you to run logic (like checking auth, logging times, or injecting DB pools) _before_ the request reaches the handler, keeping your actual handler code clean (Separation of Concerns / DRY).

### 3. HTTP Status Codes

**Q: What is the standard status code for a password mismatch?**

- **`401 Unauthorized`**: Used when credentials (password/token) are missing or invalid.
- _(Note: `403 Forbidden` is different; it means the server knows who you are, but you don't have permission to do the action)._

### 4. Production Auth Flow (JWTs vs. Sessions)

**Q: How does the flow work in production to link the `/signin` and `/verify` steps?**

- **Stateless (JWT):** At `/signin`, the server checks the DB and issues a short-lived "Pre-Auth JWT" (e.g., `status: "pending_otp"`). At `/verify`, the user submits the OTP + the Pre-Auth JWT. If valid, the server returns the Real JWT for future API access.
- **Stateful (Database):** At `/signin`, the server creates a random Session ID, stores it in the DB as `pending`, and gives it to the user (via Cookie). At `/verify`, the server checks the Cookie, verifies the OTP, and updates the DB session to `authenticated`.

### 5. Form Data vs. JSON

**Q: What is the difference between `.form()` and `.json()` in requests? They both use HashMaps.**

- **Form Data:** Serializes data like a URL query string: `To=%2B91...&Channel=sms`. This is an older HTML standard that Twilio's API requires. Requires the `application/x-www-form-urlencoded` header.
- **JSON:** Serializes data as a text object: `{"To": "+91...", "Channel": "sms"}`.
- If a server is built to parse Form Data (like Twilio), sending it JSON will result in a crash/rejection because the parsing logic fails.
