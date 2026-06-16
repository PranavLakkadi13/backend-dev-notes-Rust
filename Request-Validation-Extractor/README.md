# Request Validation in Axum

## Project Overview

This project demonstrates how to perform request-level validation in a Rust backend using the **Axum** framework. Instead of manually decoding JSON and verifying fields inside every route handler, we use a **Custom Extractor** (`ValidatedPayload<T>`).

By implementing Axum's `FromRequest` trait, the validation logic happens automatically. If the incoming data is malformed or violates validation rules (via the `validator` crate), the request is rejected early, and a custom error response is returned directly to the client.

### Key Files:

- **`src/validator.rs`**: Contains the core logic. It defines the custom `ServerError` enum (for handling rejections) and the `ValidatedPayload<T>` custom extractor.
- **`src/model.rs`**: Defines the data structures (like a request payload) with validation rules attached (e.g., `#[validate(email)]`).
- **`src/main.rs`**: Wires everything together and uses the custom extractor in the route handlers.

---

## Technical Q&A

### 1. Is `ValidatedPayload<T>` a custom extractor?

**Yes.** In Axum, any type that implements the `FromRequest` (or `FromRequestParts`) trait acts as an extractor. By implementing this trait for `ValidatedPayload<T>`, it tells Axum exactly how to take an incoming HTTP request, parse it, and hand it to the route handler.

You can use it in your handler arguments like: `ValidatedPayload(payload): ValidatedPayload<MyModel>`.

### 2. What do the generic restrictions in the `where` clause mean?

To make this extractor work for _any_ struct, we use generics (`T`), but we restrict what `T` is allowed to be:

```rust
where
    T: DeserializeOwned + Validate + Send,
    S: Send + Sync,
    Json<T>: FromRequest<S, Rejection = JsonRejection>
```

- **`T: DeserializeOwned`**: From `serde`. It means `T` can be deserialized from a string of data (like a JSON payload).
- **`T: Validate`**: From the `validator` crate. It ensures that `T` has defined validation rules and a `.validate()` method.
- **`T: Send`**: Axum processes requests asynchronously across threads. `Send` makes it safe to pass `T` between threads.
- **`S: Send + Sync`**: `S` is the application **State** (like a DB pool). Axum requires state to be thread-safe.

### 3. What does `Json<T>: FromRequest<...>` mean here?

It tells the compiler: _"Before I can create my `ValidatedPayload<T>`, I'm going to use Axum's built-in `Json<T>` extractor to do the heavy lifting of parsing the raw network request into JSON."_

In the implementation:

```rust
let Json(value) = Json::<T>::from_request(req, state).await?;
```

Instead of writing the JSON parsing logic from scratch, this delegates the logic to Axum's existing `Json` extractor.

### 4. How does the `Rejection` and custom error handling work?

```rust
type Rejection = ServerError;
```

If anything goes wrong during extraction, Axum needs to know what to return. We configured it to return the custom `ServerError` enum.

The `?` operator is used beautifully here:

1. `Json::<T>::from_request(...).await?`: If the JSON is malformed, it throws a `JsonRejection`. Because `ServerError` has `#[from]` on the `AxumJsonRejection` variant, the `?` automatically converts it.
2. `value.validate()?`: If the JSON format is fine but data fails validation, it throws `ValidationErrors`. The `?` converts it to `ServerError::ValidationError`.

Finally, because `ServerError` implements **`IntoResponse`**, Axum effortlessly turns the resulting error into an HTTP Bad Request (400) containing your standardized error message to send back to the client!

### 5. What do `#[error(transparent)]` and `#[from]` mean in the `thiserror` enum?

Both of these macros come from the **`thiserror`** crate, which reduces boilerplate when creating custom error types in Rust.

- **`#[error(transparent)]`**:
  Usually, you have to implement the `Display` trait to tell Rust how to print an error. `#[error(transparent)]` tells the compiler: _"Do not add any new error message text. Just pass the display formatting directly through to the underlying inner error."_
  If `ValidationErrors` generates the message `"email: is required"`, printing `ServerError::ValidationError` will output exactly `"email: is required"`, acting completely transparently.

- **`#[from]`**:
  This macro automatically implements the `From` trait for your enum. For example, `ValidationError(#[from] ValidationErrors)` secretly generates:
  ```rust
  impl From<ValidationErrors> for ServerError {
      fn from(err: ValidationErrors) -> Self {
          ServerError::ValidationError(err)
      }
  }
  ```
  **Why is this important?** It makes the **`?` operator** work seamlessly!
  When you write `value.validate()?`, it actually returns a `ValidationErrors` on failure. But because of `#[from]`, the `?` operator automatically wraps it inside `ServerError::ValidationError(...)` so it matches the expected return type of your function perfectly. No manual `map_err` needed!
