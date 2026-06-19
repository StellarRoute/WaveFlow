# PR payout idempotency

Duplicate merge webhooks are blocked at three layers.

## 1. Postgres pre-check

`process_merge()` calls `payout_exists()` before chain submission. Existing rows return `Conflict` ("PR N already paid").

## 2. Unique constraint

`payouts` table enforces `UNIQUE (program_id, pr_number)` in `migrations/001_init.sql`. `persist_payout()` maps insert conflicts to the same conflict error.

## 3. On-chain ProcessedPr

The escrow contract marks PRs in persistent storage (`is_pr_processed`, `mark_pr_processed`). Replayed merges return `PrAlreadyProcessed` (error code 10).

Operators replaying webhooks should expect 409 from the gateway when Postgres already recorded the payout, even if the chain tx was simulated.
