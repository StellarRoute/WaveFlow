# Local development setup

## Prerequisites

| Tool | Notes |
|------|-------|
| Rust stable | Pinned in `rust-toolchain.toml` |
| Docker | Postgres via `docker-compose` |
| Soroban CLI | Required for contract deploy (optional for API/gateway dev) |

## First-time setup

```bash
git clone https://github.com/StellarRoute/WaveFlow.git
cd WaveFlow
cp .env.example .env
docker-compose up -d
cargo build --workspace
cargo test --workspace
```

Default Postgres URL: `postgres://waveflow:waveflow@localhost:5433/waveflow`

## Run services

Terminal 1 (API, port 8081):

```bash
cargo run -p waveflow-api
```

Terminal 2 (gateway, port 8080):

```bash
cargo run -p waveflow-gateway
```

Migrations run automatically on service startup via `startup.rs`.

## Verify health

```bash
curl http://localhost:8081/health
curl http://localhost:8080/health
```

## Next steps

Follow [core-loop.md](../core-loop.md) to register a program and simulate a merged PR webhook.
