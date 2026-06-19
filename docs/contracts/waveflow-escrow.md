# waveflow-escrow contract

**Path:** `contracts/waveflow-escrow/`

Soroban escrow for Drips Wave-style bounty programs.

## Responsibilities

1. **Fund program** - Maintainer deposits assets into escrow for a program ID.
2. **Register contributor** - Map contributor identity to Stellar payout address.
3. **Record merge** - Gateway attestation after verified GitHub merge; pays `points * reward_per_point`.

## Module layout

| File | Purpose |
|------|---------|
| `lib.rs` | Contract entry and exports |
| `contract.rs` | Public contract functions |
| `types.rs` | Program, contributor, payout types |
| `storage.rs` | Persistent storage helpers |
| `events.rs` | Soroban events for indexing |
| `errors.rs` | Contract error enum |
| `test.rs` | Integration-style contract tests |

## Local contract tests

```bash
cargo test -p waveflow-escrow record_merge_pays_contributor
```

This test covers fund, register, and record_merge without Postgres or gateway.

## Deployment

Use `scripts/deploy-contract.sh` with Soroban CLI. Set resulting contract ID in `ESCROW_CONTRACT_ID`.
