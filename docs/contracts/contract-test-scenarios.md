# Contract unit test scenarios

`contracts/waveflow-escrow/src/test.rs` documents expected escrow behavior.

## Covered scenarios

| Test | Asserts |
|------|---------|
| `record_merge_pays_contributor` | Happy path payout and balance decrement |
| `duplicate_pr_is_rejected` | Second merge returns `PrAlreadyProcessed` |
| `milestone_cap_blocks_overspend` | `MilestoneCapExceeded` when cap hit |
| `invalid_repo_is_rejected` | Bad slug fails at `create_program` |
| `paused_program_rejects_merge` | `ProgramPaused` while paused |

## Setup helper

`setup()` initializes admin/gateway/token, creates a program with deposit above `MIN_DEPOSIT`, and registers a contributor before merge tests run.

Use this file as the behavioral spec when extending gateway integration tests.
