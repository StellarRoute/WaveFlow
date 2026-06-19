# API middleware and limits

The public API service layers security and observability middleware in `crates/api/src/main.rs`.

## Admin authentication

`require_admin_key` middleware checks `X-Admin-Api-Key` against `API_ADMIN_KEYS` (comma-separated). Empty configuration returns HTTP 500 to fail closed.

Admin routes are merged separately with this middleware applied only to `/admin/*`.

## Body size limit

`RequestBodyLimitLayer::new(1024 * 1024)` caps request bodies at 1 MiB.

## HTTP tracing

`TraceLayer::new_for_http()` from tower-http logs request lifecycle events at the tracing subscriber level.

See `crates/api/src/middleware.rs` and `crates/shared/src/config.rs`.
