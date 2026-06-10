# Cross-Origin Resource Sharing (CORS)

This folder contains a Rust-based demo demonstrating how **Cross-Origin Resource Sharing (CORS)** works using the [Axum](https://github.com/tokio-rs/axum) web framework and `tower-http`.

---

## 💡 What is CORS?

**CORS** is a browser-enforced security mechanism. By default, browsers implement the **Same-Origin Policy (SOP)**, which restricts a web page/script from making requests to a different domain (origin) than the one that served it.

An **Origin** is defined as the combination of:
1. **Protocol** (e.g., `http` vs `https`)
2. **Domain/Host** (e.g., `localhost` vs `example.com`)
3. **Port** (e.g., `:3000` vs `:3001`)

If any of these three elements differ between the frontend website and the API server, it is a **Cross-Origin Request**.

---

## 🛠️ How this Demo Works

The example in [src/main.rs](src/main.rs) starts two servers concurrently:

1. **Frontend Server (Port `3000`)**:
   - Serves a basic HTML page.
   - Run a JavaScript `fetch()` script that requests data from the backend server (`http://localhost:3001/data`) and sends an `Authorization` header.

2. **Backend Server (Port `3001`)**:
   - Exposes a `/data` endpoint returning JSON.
   - Configures a CORS policy using `tower_http::cors::CorsLayer`.

---

## 🔍 Understanding the CORS Configuration in Rust

In the backend server, CORS is configured as follows:

```rust
let app = Router::new().route("/data", get(data)).layer(
    CorsLayer::new()
        // 1. Allow the frontend origin
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        // 2. Allow the non-standard "Authorization" header
        .allow_headers([http::header::AUTHORIZATION]),
);
```

### 1. Allow Origin
Without `.allow_origin("http://localhost:3000"...)`, the browser loading the frontend code would block the response from the backend. The server sends the `Access-Control-Allow-Origin: http://localhost:3000` header in response, telling the browser it is safe to share the data.

### 2. Allow Headers (Preflight Requests)
Because the frontend sends a custom/non-simple header (`Authorization`):
1. The browser first sends a preflight request using the HTTP **`OPTIONS`** method.
2. The server receives this and checks if `Authorization` is allowed via `.allow_headers(...)`.
3. The server responds with `Access-Control-Allow-Headers: authorization`.
4. The browser verifies the response and finally fires the real `GET` request.

---

## 🚀 How to Run

1. Navigate to this directory in your terminal:
   ```bash
   cd CORS
   ```
2. Run the application:
   ```bash
   cargo run
   ```
3. Open your browser and go to `http://localhost:3000`.
4. Open the **Browser Developer Tools** (F12 or right-click -> Inspect) and view the **Console** tab. You should see the backend's JSON response:
   ```json
   "{message: Hello World}"
   ```
