# Postgres vs on-chain balance drift

`programs.escrow_balance` and `milestone_spent` columns exist in Postgres but are not updated on every gateway payout today.

## Postgres defaults

`create_program` admin route inserts programs with zeroed balance fields. Gateway `persist_payout()` writes the `payouts` row but does not UPDATE `programs.escrow_balance`.

## On-chain source of truth

`record_merge` in the escrow contract decrements escrow and increments milestone spend atomically on Soroban.

## Operational guidance

Treat Postgres balances as bootstrap metadata only until a sync job or indexer backfills from chain events (`MergeRecordedEvent`, `FundedEvent`).

Reconciliation dashboards should read contract state or indexed events, not stale Postgres columns alone.

See `migrations/001_init.sql`, `crates/api/src/routes/admin.rs`, and `contracts/waveflow-escrow/src/contract.rs`.
