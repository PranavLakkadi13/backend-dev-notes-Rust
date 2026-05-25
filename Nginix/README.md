# Docker & NGINX Concepts

## Quick Reference: Building Docker Image

```bash
sudo docker build . -t app:latest-nginx
```

Creates a Docker image from the Dockerfile. The `.` tells Docker to use the current directory, `-t` tags it with a name.

---

## Questions & Doubts

> **Q1:** Dockerfile creates a Docker image? Pulls Rust, builds `main.rs`, copies code to `/usr/src/app`, then builds & stores it?
> **Q2:** Docker Compose orchestrates services? `rust_app_instance_1` builds image, has env vars, exposes ports, same for `instance_2`?
> **Q3:** NGINX loads volumes (local nginx.conf → image config)? `depends_on` ensures Rust services run first?
> **Q4:** What does NGINX do? Why use it as a backend dev?
> **Q5:** Can NGINX load-balance different codebases? Or should both services be identical?
> **Q6:** What is a reverse proxy?
> **Q7:** How do the `proxy_set_header` lines work in `nginx.conf`?

---

## Explanations

**Dockerfile Basics:**

- `FROM rust:latest` - pulls Linux + Rust environment
- `WORKDIR /usr/src/app` - sets container working directory
- `COPY . .` - copies your code into container
- `RUN cargo build --release` - compiles code in isolation

**Docker Compose:**
Orchestrates multiple services. Both `rust_app_instance_1` and `rust_app_instance_2` use the same codebase (`build: ../`) with different env vars (`Server1` vs `Server2`). The `volumes` section binds your `nginx.conf` to the container's `/etc/nginx/nginx.conf`. `depends_on` ensures Rust services start before NGINX.

**What NGINX Does:**

- **Reverse Proxy:** Sits in front of Rust apps, protecting them from direct internet exposure
- **Load Balancer:** Distributes requests across multiple instances (Round-Robin by default)
- **Router/API Gateway:** Routes different URLs to different services (Microservices)
- **Static File Server:** Efficiently serves images/CSS without burdening backend
- **HTTPS Termination:** Handles encryption; backends get plain HTTP

**Reverse Proxy Concept:**
Forward Proxy (VPN) protects _clients_. Reverse Proxy (NGINX) protects _backends_. Users never see your Rust app's real IP/port—only NGINX. Benefits: Security, error handling, centralized SSL/TLS.

**Different Codebases:**
If services differ (User Service vs Payment Service), NGINX acts as a **Router**, not Load Balancer. It reads URLs: `/users` → Instance 1, `/payments` → Instance 2. This is **Microservices Architecture**.

**`nginx.conf` Location Block:**

```nginx
location / {
    proxy_pass http://axum;              # Forward to axum pool
    proxy_set_header HOST $host;         # Preserve original domain
    proxy_set_header X-Real-IP $remote_addr;  # User's real IP
    proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;  # IP chain
    proxy_set_header X-Forwarded-Proto $scheme;  # http or https?
}
```

**The Identity Crisis:** When NGINX forwards a request, your Rust app thinks NGINX _is_ the user. These headers restore the truth:

- `HOST` - original domain the user typed
- `X-Real-IP` - user's true IP (critical for banning/rate-limiting)
- `X-Forwarded-For` - full IP chain through proxies
- `X-Forwarded-Proto` - tells Rust if user is on http or https
