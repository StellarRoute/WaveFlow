# Architecture overview

WaveFlow connects GitHub merge events to on-chain bounty payouts on Stellar/Soroban.

## Data flow

```
GitHub PR merge
    -> Gateway (HMAC verify, parse event)
    -> Postgres (resolve program + contributor)
    -> Soroban escrow contract (record_merge / payout)
    -> Postgres audit (payout row + tx hash)
    -> API (read programs and payout history)
```

## Services

| Binary | Crate | Default port |
|--------|-------|--------------|
| `waveflow-gateway` | `crates/gateway` | 8080 |
| `waveflow-api` | `crates/api` | 8081 |

## Shared infrastructure

- **Postgres:** programs, contributors, payouts, idempotency
- **Soroban RPC:** contract invocation from gateway attestation module

## Contract

`contracts/waveflow-escrow` holds program funding, contributor registration, and merge attestation logic. See [contracts/waveflow-escrow.md](../contracts/waveflow-escrow.md).

## Deployment

Render blueprint: `render.yaml` (gateway + API + managed Postgres). See [deployment/render.md](../deployment/render.md).
