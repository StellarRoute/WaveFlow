# Testing guide

## Run all workspace tests

```bash
cargo test --workspace
```

## Contract tests

```bash
cargo test -p waveflow-escrow
```

Key test: `record_merge_pays_contributor` validates fund, register, and payout without external services.

## Shared crate tests

Config parsing tests in `crates/shared/src/config.rs` verify comma-separated `API_ADMIN_KEYS`.

## Webhook unit tests

`crates/gateway/src/webhook.rs` tests:

- HMAC verification (valid and invalid signatures)
- Merge event parsing (merged vs closed-without-merge, wrong base branch)

## CI

`.github/workflows/ci.yml` runs fmt, clippy, and tests on push/PR to `main`.

## Integration testing locally

1. Start Postgres: `docker-compose up -d`
2. Run API and gateway
3. Register program and contributor via admin API
4. POST signed webhook using [local-webhook-testing.md](local-webhook-testing.md)
5. Query `GET /api/v1/programs/:id/payouts`
