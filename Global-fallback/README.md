# Axum Fallback Handlers & Precedence

When writing backend services in Rust using the [axum](https://docs.rs/axum/latest/axum/) framework, it is important to understand how fallback handlers work when routing.

## ⚠️ Multiple Fallbacks on a Single Router
If you try to chain multiple `.fallback(...)` methods on a single router, they **do not** form a chain of precedence where the bottom one is triggered first. Instead, **the last fallback defined overwrites any previous fallbacks**.

### Incorrect Example
```rust
let app = axum::Router::new()
    .route("/health", get(health_handler))
    .fallback(fallback_handler)
    .fallback(fallback_handler2); // ❌ This overwrites `fallback_handler` entirely. Only `fallback_handler2` will run!
```

---

## 🛠️ Correct Way: Nested Fallbacks
If you need different fallback handlers for different route prefixes (e.g., one for `/api/...` and a global one for static assets or 404 pages), you should **nest** routers. Each nested router can have its own fallback.

### Code Snippet
```rust
use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // 1. Define fallback for API routes
    let api_router = Router::new()
        .route("/users", get(get_users))
        .fallback(api_fallback_handler); // Triggers for any unmatched /api/* route

    // 2. Nest it under the main router and define a global fallback
    let app = Router::new()
        .nest("/api", api_router)
        .route("/health", get(health_handler))
        .fallback(global_fallback_handler); // Triggers for any unmatched route outside /api (e.g. /health-typo)

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_handler() -> &'static str {
    "OK"
}

async fn get_users() -> &'static str {
    "[{ \"id\": 1, \"name\": \"Alice\" }]"
}

// Fallback when a route under /api is not found (e.g., GET /api/not-existing)
async fn api_fallback_handler() -> (axum::http::StatusCode, &'static str) {
    (axum::http::StatusCode::NOT_FOUND, "API Route not found")
}

// Global fallback for any other route not found (e.g., GET /unknown)
async fn global_fallback_handler() -> (axum::http::StatusCode, &'static str) {
    (axum::http::StatusCode::NOT_FOUND, "Global 404: Page not found")
}
```
