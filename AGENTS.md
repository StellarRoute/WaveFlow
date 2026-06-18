# AGENTS.md

Guidance for AI agents working in the WaveFlow repository.

## What this repo is

WaveFlow is a Stellar/Soroban bounty escrow system. Maintainers fund programs linked to GitHub repos; merged PRs trigger gateway attestations that pay contributors on-chain.

## Common commands

From repo root:

```bash
docker-compose up -d
cargo build --workspace
cargo test --workspace
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo run -p waveflow-gateway
cargo run -p waveflow-api
```

Single test:

```bash
cargo test -p waveflow-escrow record_merge_pays_contributor
cargo test -p waveflow-gateway verify_github_signature
```

## Required runtime configuration

- Gateway: `DATABASE_URL`, `GITHUB_WEBHOOK_SECRET`, `SOROBAN_RPC_URL`, `GATEWAY_SECRET_KEY`
- API: `DATABASE_URL`, `API_ADMIN_KEYS`, `PORT`

## High-value files

- `contracts/waveflow-escrow/src/contract.rs` — on-chain escrow logic
- `crates/gateway/src/webhook.rs` — GitHub webhook handling
- `crates/gateway/src/attestation.rs` — merge → chain payload
- `crates/api/src/routes/` — REST endpoints
- `migrations/` — Postgres schema
