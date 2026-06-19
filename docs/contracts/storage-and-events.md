# Contract storage and events

Implementation: `contracts/waveflow-escrow/src/storage.rs` and `events.rs`.

## Storage model (conceptual)

Programs are keyed by on-chain program ID. Each program tracks:

- Maintainer address
- Escrow balance
- Reward per point
- Optional milestone cap and spent amount
- Registered contributors (GitHub identity to Stellar address)

Contributors must be registered before merge payouts succeed.

## Events

Contract emits Soroban events on:

- Program funded (amount, program ID)
- Contributor registered
- Merge recorded (PR reference, payout amount, recipient)

Events support future indexers and audit reconciliation against Postgres payout rows.

## Error cases

See `errors.rs` for conditions such as:

- Insufficient escrow balance
- Unknown program or contributor
- Duplicate merge attestation (idempotency)
- Milestone cap exceeded

Gateway should map contract errors to logged failures and non-retryable HTTP responses where appropriate.
