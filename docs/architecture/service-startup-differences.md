# Gateway vs API startup

The two binaries share migrations but differ in port binding and health checks.

## Gateway (`crates/gateway/src/main.rs`)

- Runs `sqlx::migrate!` on boot
- Listens on `GATEWAY_PORT` from config
- Does **not** call `run_startup_checks`
- Exposes `/health`, `/webhooks/github`, `/metrics`

## API (`crates/api/src/main.rs`)

- Runs migrations then `startup::run_startup_checks` (DB ping, config validation)
- Port: `PORT` env override, else `API_PORT` from config
- Layers admin auth, body limit, and TraceLayer
- Exposes public REST, admin routes, `/metrics`

Platform hosts (Render, Fly) typically set `PORT`; local dev uses config defaults.

See `crates/shared/src/config.rs` for `gateway_port` and `api_port`.
