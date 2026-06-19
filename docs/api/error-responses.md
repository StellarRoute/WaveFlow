# Error response mapping

`WaveFlowError` converts to HTTP status codes consistently across gateway and API.

## Status table

| Variant | HTTP |
|---------|------|
| `Unauthorized` | 401 |
| `Validation`, `Webhook` | 400 |
| `NotFound` | 404 |
| `Conflict` | 409 |
| `Config`, `Database`, `Chain`, `Internal` | 500 |

## Response shape

Gateway `map_error()` returns JSON `{ "error": "<message>" }` with the mapped status.

API routes use `ApiError` implementing `IntoResponse` with the same status mapping via `http_status_code()`.

Clients should treat 200 ignored webhooks separately from 4xx/5xx admin API failures.

See `crates/shared/src/error.rs`.
