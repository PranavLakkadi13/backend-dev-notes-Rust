# Distributed Rate Limiting with Redis in Rust

This directory contains notes and implementation details for a distributed rate limiter using Redis and Rust.

## Why Redis over Nginx?

Traditionally, Nginx is used for rate limiting (e.g., using `limit_req_zone`). However, Nginx stores these rate-limiting counters in its local memory. If you have multiple application servers running behind a load balancer, they will not share these counters, leading to inconsistent rate limiting.

Redis solves this by acting as a centralized, fast, in-memory data store. Multiple application servers can talk to the same Redis instance, ensuring that you have a single source of truth and making the rate limiting truly **distributed**.

## When Does Traditional Rate Limiting Work?

If you have **only one Load Balancer** doing the rate limiting (not multiple application servers), then traditional rate limiting works perfectly fine. All requests go through that single point, so the counter is always accurate. However, scaling challenges arise in two scenarios:

1. **Multiple Application Servers Behind One Load Balancer:** Each server maintains its own counter. A user making 30 requests in a minute could get 10 through each server, bypassing your limit of 10 per minute.
2. **Multiple Load Balancers (High Availability):** If you add a second load balancer for redundancy or to handle more traffic, you face the same problem—they don't share state.

The workaround in traditional setups is to use "Sticky Sessions" (route all requests from the same IP to the same server), but this defeats the purpose of load balancing since one user can overload a single server while others sit idle.

## What is Redis?

Redis stands for **REmote DIctionary Server**. It is an open-source, in-memory key-value database that operates like a super-fast cache.

**Key Characteristics:**

- **In-Memory:** Data lives in RAM, providing microsecond-level latency.
- **Optional Persistence:** You can configure Redis to write to disk (RDB snapshots or AOF logs) for data durability across restarts.
- **Single-Threaded:** Operations are atomic, meaning no race conditions occur when multiple servers access the same key simultaneously.
- **Key-Value Pairs:** Every piece of data is stored with a unique key (e.g., `user:1:recent_ips`) and an associated value.

**Example command structure:**

```
LPUSH user:1:recent_ips "192.168.1.1" "192.168.1.2" "192.168.1.3"
       ↑                   ↑
    command                key                          values
```

## Redis Data Types

Redis supports several value types, not just strings:

- **Strings**: Numbers, text, or binary data. Used in your rate limiter for storing the request count.
- **Lists**: Ordered collections. Example: `LPUSH user:1:recent_ips "192.168.1.1"` stores a history of IPs.
- **Sets**: Unique collections. Example: `SADD blocked_ips "192.168.1.100"` maintains a list of blocked IPs.
- **Hashes**: Dictionaries with multiple fields. Example: `HSET rate_limit:192.168.1.5 count 42 first_request 1717504800` stores both count and timestamp.
- **Sorted Sets**: Ordered by score. Example: Track which IPs made the most requests.

For your rate limiting use case, the **String type** (storing an integer counter) is perfect and efficient.

## Common Redis Commands for Rate Limiting

- `INCR key`: Increments the integer value of a key by 1. It is atomic, meaning multiple servers can call it at the exact same moment without race conditions.
- `EXPIRE key seconds`: Sets a time-to-live (TTL) on a key so it deletes itself automatically (useful for resetting a "requests per minute" window).
- `GET key`: Retrieves the current count.
- `SET key value`: Stores a value with a key.
- `SET_EX key value seconds`: Atomically sets a key-value pair with expiration time.

## The Logic (Fixed Window Algorithm)

We use Redis as a service to hold a counter of the requests stemming from an IP address. The IP address acts as the key (e.g., `rate_limit:192.168.1.5`).

1. A request comes in.
2. We run `INCR` on that key.
3. If it is the first request (the count is 1), we also set an `EXPIRE` time for the key (e.g., 60 seconds).
4. If the returned count is greater than our defined limit, the server rejects the request (typically returning an HTTP 429 Too Many Requests error).

## Connecting to Redis with Docker

In the Rust implementation, the connection is established like this:

```rust
let client = redis::Client::open("redis://127.0.0.1/").unwrap();
```

- **Default Port**: When the `redis` crate sees the `redis://` protocol scheme without an explicit port, it automatically defaults to Redis's standard port, `6379`. It is equivalent to writing `redis://127.0.0.1:6379/`.
- **Docker Port Mapping**: To run Redis, we use a Docker command like `docker run -p 6379:6379 -d redis`. The `-p 6379:6379` flag handles port mapping (HostPort:ContainerPort). It maps port `6379` on your Host Machine (Mac, `127.0.0.1`) to port `6379` inside the container. Docker intercepts the connection made by the Rust application and tunnels the traffic straight into the container where the Redis server is listening.

## Key Clarifications & Doubts Answered

### How Does Redis Work as a Key-Value Database?

Redis is fundamentally a **key-value database that lives in RAM with optional persistence to storage**. Every entry consists of:

- **Key**: The identifier/name (e.g., `user:1:recent_ips`, `rate_limit:192.168.1.5`)
- **Value**: The data stored (e.g., a string, list, hash, etc.)

The colon (`:`) in key names is just a naming convention for organization—Redis doesn't enforce it. You could name it `user_1_recent_ips` or `mykey123`.

### What Happens Without Persistence?

If Redis is running **without persistence** and the server crashes, all data in RAM is lost. However, you can enable persistence to save data to disk periodically (RDB snapshots) or in real-time (AOF logs), ensuring your data survives a restart.

### Our Rate Limiter Implementation

In your code, you store the IP as the key and an integer counter as the value:

```rust
let _: () = con.set_ex(&ip, 1, 60).unwrap();
```

- **Key**: The IP address (e.g., `192.168.1.5`)
- **Value**: `1` (the initial count)
- **Expiration**: `60` seconds (Redis automatically deletes this entry after 60 seconds)

This is the simplest and most efficient approach for rate limiting.
