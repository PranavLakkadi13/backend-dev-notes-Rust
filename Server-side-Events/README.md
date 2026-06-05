# Server-Sent Events (SSE) in Rust

Server-Sent Events (SSE) is a push technology where the server establishes a long-lived HTTP connection to push real-time updates to the client.

---

## 💡 SSE vs. WebSockets: Key Advantages

Even though both can stream data, SSE is often preferred over WebSockets for server-to-client notifications or dashboards due to these advantages:

1. **Protocol Simplicity**: SSE runs over standard HTTP/HTTPS (`text/event-stream`). It requires no protocol upgrades or custom ports. It works seamlessly with standard firewalls, reverse proxies (like Nginx), and load balancers.
2. **Built-in Auto-Reconnection**: The client-side browser API (`EventSource`) automatically handles reconnection if the stream gets interrupted. With WebSockets, you have to implement this logic manually.
3. **Unidirectional Design**: If the client only needs to receive data (e.g., live feeds, notifications, stock tickers), SSE is cleaner and less resource-heavy than a bidirectional WebSockets connection.
4. **HTTP/2 & HTTP/3 Multiplexing**: Under HTTP/2, browsers can multiplex multiple SSE streams over a single TCP connection, preventing the 6-connection browser limit per domain.

---

## ❓ FAQ & Revision Notes

### Q1: Is SSE just "polling" the server at fixed intervals?
**No.** SSE is a **server-push** technology. The client opens a *single* HTTP connection, and the server keeps it open. The server pushes data immediately when it's ready. There is no periodic request/response cycle like in short or long polling.

### Q2: How does the client handle idle periods (e.g., if the server only pushes data every 5 seconds)?
In async Rust (using `tokio` and streams), you consume the stream with a loop like this:
```rust
while let Some(chunk) = stream.try_next().await.unwrap() {
    // Process chunk...
}
```

* **Does the loop exit when the stream is empty?**
  No. The loop only exits when the stream is closed by the server (returns `None`).
* **Does it block or spin/busy-wait?**
  No. The `.await` keyword suspends the execution of the task. It yields control back to the Tokio executor and registers the socket with the OS. It consumes **0% CPU** during the idle time. When the OS detects incoming data on the network card, Tokio wakes the task back up to process the new chunk.
