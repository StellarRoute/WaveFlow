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
  -H "x-api-key: dev-ad...ion" \
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
  -H "x-api-key: dev-ad...ion" \
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

---

## Testnet sequence (full merge → on-chain payout)

The local sequence above uses the gateway's `dry-run` path when `ESCROW_CONTRACT_ID`
or `GATEWAY_SECRET_KEY` is unset. To validate the full loop on Stellar testnet,
follow the steps below. They assume you have the Soroban CLI installed
(`cargo install --locked soroban-cli` or `brew install soroban`) and a funded
testnet identity (`soroban keys generate --fund alice --network testnet`).

### 7. Install the WASM target and build the escrow contract

```bash
rustup target add wasm32-unknown-unknown
cargo build --release -p waveflow-escrow --target wasm32-unknown-unknown
```

The compiled WASM is at
`target/wasm32-unknown-unknown/release/waveflow_escrow.wasm`. The deploy script
wraps the call below and is the recommended entry point:

```bash
./scripts/deploy-contract.sh
```

### 8. Deploy and initialize the contract on testnet

`scripts/deploy-contract.sh` is the wrapper; the equivalent `soroban` calls are:

```bash
# Deploy (returns the contract id; capture it as $ESCROW_CONTRACT_ID)
CONTRACT_ID=$(soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/waveflow_escrow.wasm \
  --source alice \
  --rpc-url https://soroban-testnet.stellar.org \
  --network testnet)

# Initialize the program with a maintainer address and reward ratio
soroban contract invoke \
  --id "$CONTRACT_ID" \
  --source alice \
  --rpc-url https://soroban-testnet.stellar.org \
  --network testnet \
  -- initialize \
  --maintainer G... \
  --reward_per_point 100
```

After the call succeeds, copy the value of `CONTRACT_ID` into `.env` (see
step 9). The testnet explorer URL will look like
`https://stellar.expert/explorer/testnet/contract/<CONTRACT_ID>`.

### 9. Configure the gateway with live testnet credentials

Edit `.env` and set the variables that flip the gateway out of dry-run mode.
The full template lives in `.env.example`; the testnet-only block is:

```bash
# Soroban / Stellar (testnet)
SOROBAN_NETWORK=testnet
SOROBAN_RPC_URL=https://soroban-testnet.stellar.org
NETWORK_PASSPHRASE="Test SDF Network ; September 2015"
ESCROW_CONTRACT_ID=<paste from step 8>
GATEWAY_SECRET_KEY=<funded testnet secret, e.g. SD...>
```

Restart the gateway so the new env values are picked up:

```bash
docker-compose restart waveflow-gateway
```

`crates/gateway/src/attestation.rs::submit_attestation` will now route through
`Soroban` and return a real transaction hash (no more `simulated-tx-*` or
`dry-run-*` placeholders). Watch the structured logs for the line
`submitting Soroban record_merge transaction` with `contract`,
`program_id`, and `pr` fields.

### 10. Re-register program and contributor (live testnet program id)

The admin API takes the **on-chain** `program_id` (the `u64` you initialized
the contract with, not the Postgres UUID). Use the same value as the
`on_chain_program_id` field in the program registration call from step 3.
Contributor registration is identical to step 4.

### 11. Trigger a real merge

Open a PR against the registered repo, get it merged, and watch the gateway
log stream for the attestation pipeline. The Postgres `webhook_events` table
will show status `delivered` and the `payouts` table will be populated with
the on-chain transaction hash.

### 12. Verify the on-chain payout

Two ways to verify end-to-end:

```bash
# API path
curl http://localhost:8081/api/v1/programs/<uuid>/payouts
```

The response includes a `tx_hash` field. Open the matching record on
`https://stellar.expert/explorer/testnet/tx/<tx_hash>` and confirm the
`record_merge` invocation paid `points × reward_per_point` to the
contributor Stellar address you registered in step 4.

```bash
# Direct contract query (optional)
soroban contract invoke \
  --id "$ESCROW_CONTRACT_ID" \
  --source alice \
  --rpc-url https://soroban-testnet.stellar.org \
  --network testnet \
  -- get_payout \
  --program_id <on_chain_program_id> \
  --pr_number <pr_number>
```

The on-chain result must match the API response; any divergence means a
missing or stale row in the off-chain audit trail.

### 13. Idempotency and replay safety

The gateway enforces two layers of idempotency before paying out:

- `webhook_events` has a unique constraint on `delivery_id` (per GitHub
  delivery).
- `payouts` has a unique constraint on `(program_id, pr_number)` (per merged
  PR).

Re-delivering the same GitHub webhook or re-submitting the same PR does not
double-pay. Failed attestations surface in `webhook_events.status` and
`payouts.error_message`; the on-chain escrow balance decreases by exactly the
paid amount (see `load_reward_per_point` + `increment_milestone_spent` in
`crates/gateway/src/attestation.rs`).

### 14. Common testnet pitfalls

- **Wrong network passphrase** — gateway logs `record_merge transaction`
  with the right contract id but submission fails. Confirm
  `NETWORK_PASSPHRASE="Test SDF Network ; September 2015"` matches the
  `soroban` CLI default for `--network testnet`.
- **Underfunded gateway signer** — `GATEWAY_SECRET_KEY` must hold at least
  the rent + fee budget for the contract invocation. Friendbot top-up:
  `curl https://friendbot.stellar.org?addr=<G_ADDRESS>`.
- **Contract not initialized** — calling `record_merge` before
  `initialize` reverts. Re-run step 8's `initialize` invocation.
- **Stale `ESCROW_CONTRACT_ID`** — after redeploying, the gateway still
  points at the old contract id and returns `dry-run-*` hashes. Restart
  the gateway with the new value.
- **`waveflow-escrow` build profile** — the deploy script uses
  `--release`; debug builds can exceed the Soroban WASM size budget.

See `docs/PRD.md` Section 6 (operational flow) and the gateway integration
tests under `crates/gateway/tests/` for end-to-end fixtures.
