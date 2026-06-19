# Payout calculation

Merge rewards are computed off-chain before Soroban attestation.

## Points default

`parse_merge_event()` in `webhook.rs` sets `points = 1` for every merged PR today. Future versions may read labels or config.

## Amount formula

```
payout_amount = points * reward_per_point
```

`load_reward_per_point()` reads `programs.reward_per_point` from Postgres. `persist_payout()` guards i128 overflow when multiplying.

## Attestation payload

`build_attestation()` sends `program_id` (on-chain numeric id), `github_username`, `pr_number`, and `points` to `record_merge`.

See `crates/gateway/src/routes.rs` (`process_merge`) and `crates/gateway/src/attestation.rs`.
