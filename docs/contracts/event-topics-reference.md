# Soroban event topics

Indexers can subscribe to contract events defined in `events.rs`.

## Event structs

| Event | Fields |
|-------|--------|
| `ProgramCreatedEvent` | `program_id`, `maintainer`, `github_repo`, `reward_per_point` |
| `FundedEvent` | `program_id`, `amount`, `new_balance` |
| `ContributorRegisteredEvent` | `program_id`, `github_username`, `stellar_address` |
| `MergeRecordedEvent` | `program_id`, `github_username`, `pr_number`, `points`, `payout_amount` |
| `ProgramPausedEvent` | `program_id` |
| `ProgramResumedEvent` | `program_id` |

## Publishing

`contract.rs` publishes with `env.events().publish((symbol_short!("..."),), payload)` using short topic symbols for each lifecycle action.

Match on-chain topics when building Horizon or RPC event filters for payout reconciliation.
