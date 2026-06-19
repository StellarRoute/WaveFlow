# Environment variables

Loaded by `crates/shared/src/config.rs` via `AppConfig::from_env()`.

## Required

| Variable | Description |
|----------|-------------|
| `DATABASE_URL` | Postgres connection string for audit trail |
| `GITHUB_WEBHOOK_SECRET` | Shared secret for HMAC verification |

## Soroban / Stellar

| Variable | Default | Description |
|----------|---------|-------------|
| `SOROBAN_RPC_URL` | `https://soroban-testnet.stellar.org` | RPC endpoint |
| `NETWORK_PASSPHRASE` | `Test SDF Network ; September 2015` | Network identifier. Use `Public Global Stellar Network ; September 2015` with mainnet RPC. |
| `ESCROW_CONTRACT_ID` | none | Deployed contract ID (required for live attestation) |
| `GATEWAY_SECRET_KEY` | none | Stellar secret key for gateway signing |

## API auth

| Variable | Default | Description |
|----------|---------|-------------|
| `API_ADMIN_KEYS` | empty | Comma-separated keys for admin routes |

Send admin key as header: `x-api-key: <key>`

## Ports

| Variable | Default | Description |
|----------|---------|-------------|
| `GATEWAY_PORT` | `8080` | Gateway bind port |
| `API_PORT` | `8081` | API bind port |
| `PORT` | falls back to gateway | Used by Render for gateway |

## Observability

| Variable | Description |
|----------|-------------|
| `RUST_LOG` | Tracing filter (see `.env.example`) |
| `OTEL_EXPORTER_OTLP_ENDPOINT` | Optional OpenTelemetry collector |
