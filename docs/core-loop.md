# End-to-end walkthrough of the WaveFlow merge → payout core loop.

## Overview

1. Maintainer creates an on-chain program and funds escrow via Soroban.
2. Maintainer registers program metadata and contributors in Postgres (via API).
3. GitHub sends a `pull_request` webhook when a PR merges.
4. Gateway verifies HMAC, resolves program + contributor, submits attestation.
5. Soroban contract pays `points × reward_per_point` to the contributor wallet.

## Local sequence

### 1. Start dependencies

```bash
cp .env.example .env
docker-compose up -d
```

### 2. Run migrations (automatic on service start)

```bash
cargo run -p waveflow-api
cargo run -p waveflow-gateway
```

### 3. Register program (admin API)

```bash
curl -X POST http://localhost:8081/api/v1/admin/programs \
  -H "Content-Type: application/json" \
  -H "x-api-key: dev-admin-key-change-in-production" \
  -d '{
    "on_chain_program_id": 1,
    "github_repo": "StellarRoute/WaveFlow",
    "maintainer_address": "G...",
    "reward_per_point": 100
  }'
```

### 4. Register contributor

```bash
curl -X POST http://localhost:8081/api/v1/admin/contributors \
  -H "Content-Type: application/json" \
  -H "x-api-key: dev-admin-key-change-in-production" \
  -d '{
    "program_id": "<uuid from step 3>",
    "github_username": "alice",
    "stellar_address": "G..."
  }'
```

### 5. Simulate GitHub webhook

Compute HMAC-SHA256 of the JSON body with `GITHUB_WEBHOOK_SECRET`, send to gateway:

```bash
curl -X POST http://localhost:8080/webhooks/github \
  -H "Content-Type: application/json" \
  -H "X-GitHub-Event: pull_request" \
  -H "X-Hub-Signature-256: sha256=<digest>" \
  -d @fixtures/merged_pr.json
```

### 6. Verify payout audit trail

```bash
curl http://localhost:8081/api/v1/programs/<uuid>/payouts
```

## On-chain proof (contract tests)

```bash
cargo test -p waveflow-escrow record_merge_pays_contributor
```

This test covers fund → register → record_merge without external services.
