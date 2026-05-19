# WebSockets Overview

## What are WebSockets?

WebSockets provide a full-duplex (bidirectional) communication channel over a single, long-lived TCP connection. Unlike traditional HTTP, where a client must initiate a request to get a response, a WebSocket connection remains open, allowing both the client and the server to send messages to each other at any time.

## Why are they used?

WebSockets are used primarily for real-time applications where low latency and continuous data updates are required. They eliminate the overhead of traditional HTTP polling (where the client repeatedly asks the server for new data) and allow the server to push data exactly when it's available.

- **Common Use Cases:**
  - Real-time trading and financial tickers
  - Live chat applications
  - Multiplayer online games
  - Collaborative document editing
  - Live sports scoreboards or notifications

## Edge Cases and Design Considerations

When designing a system that relies on WebSockets, you must account for the fact that persistent connections are more complex to manage than stateless HTTP requests.

1. **Connection Drops & Network Instability:**
   - Connections will inevitably drop due to network switches, poor cell reception, or server deployments. Your client must implement robust **auto-reconnection** logic (often with exponential backoff).
2. **Zombie Connections (Heartbeats):**
   - Sometimes a connection drops silently, and the server/client isn't notified. You should implement a **Ping/Pong (Heartbeat)** mechanism. If the server doesn't receive a Pong within a certain timeframe, it should actively kill the connection to free up resources.
3. **Load Balancing & Horizontal Scaling:**
   - Because WebSockets are stateful, a user's connection lives on a specific server instance. If you have multiple server instances, you need a mechanism (like a **Pub/Sub backend e.g., Redis**) to broadcast messages across all server instances so that a message intended for User A (on Server 1) can be routed from Server 2. Load balancers must also be configured to support sticky sessions or long-lived connections.
4. **Message Ordering & Guarantee:**
   - While TCP guarantees order, if a connection drops and the client reconnects, messages sent during the downtime might be lost. You may need a mechanism to queue messages or resync state upon reconnection.
5. **Security & Authentication:**
   - Always use `wss://` (WebSocket Secure) to encrypt data in transit. Also, since there are no standard headers like in HTTP for every frame, authentication is usually handled during the initial HTTP upgrade handshake (via tokens) or via the first WebSocket message sent by the client.


## Common Questions and Clarifications

**Q: Do WebSockets open a continuous connection until closed to share messages?**
**A:** Yes. A WebSocket starts as a standard HTTP "upgrade" request and stays continuously open. It remains active until either the client or the server explicitly decides to close it, allowing for continuous data flow without having to re-establish connections.

**Q: Are WebSockets always bidirectional, or can they be unidirectional?**
**A:** WebSockets are inherently **bidirectional (full-duplex)**, meaning both the client and server can send messages independently at any time. However, you can use them in a **unidirectional** manner depending on your application logic. For example, a trading app could have clients connect and just listen to continuous price updates from the server without ever sending messages back.

**Q: Is it polling-based, needing a request to send a response?**
**A:** No, WebSockets are **NOT polling-based**. In traditional HTTP (polling), the client repeatedly asks, "Do you have new data?". WebSockets eliminate this round-trip. Once the connection is established, the server can **push** data to the client whenever new data is available. You do not need to wait for a request from the client to send a response.