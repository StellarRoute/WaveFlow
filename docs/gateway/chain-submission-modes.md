# Gateway chain submission modes

`submit_attestation()` in `crates/gateway/src/attestation.rs` bridges Postgres-resolved merge events to Soroban.

## When RPC credentials are present

If both `ESCROW_CONTRACT_ID` and `GATEWAY_SECRET_KEY` are set, the gateway logs an intent to submit `record_merge` and returns a deterministic placeholder hash:

```
simulated-tx-{program_id}-{pr_number}
```

Full Soroban RPC client wiring is deployment-specific; production should replace the placeholder with a signed transaction.

## Dry-run mode

When either credential is missing, the function logs a warning and returns:

```
dry-run-{program_id}-{pr_number}
```

No chain call is attempted. Local development and CI use this path.

## Configuration

| Variable | Role |
|----------|------|
| `SOROBAN_RPC_URL` | Passed to submission helper |
| `ESCROW_CONTRACT_ID` | Contract address |
| `GATEWAY_SECRET_KEY` | Signing key for gateway role |

See `crates/shared/src/config.rs` for env parsing.
