# Contract error codes

`ContractError` in `errors.rs` maps to `u32` return codes on failed invocations.

| Code | Variant | Typical cause |
|------|---------|---------------|
| 1 | AlreadyInitialized | Second `initialize` |
| 2 | NotInitialized | Call before setup |
| 3 | Unauthorized | Wrong caller role |
| 4 | InvalidRepo | Malformed GitHub slug |
| 5 | ProgramNotFound | Unknown program id |
| 6 | ContributorNotFound | Unregistered contributor |
| 7 | ContributorAlreadyRegistered | Duplicate registration |
| 8 | InsufficientEscrow | Balance too low for payout |
| 9 | ProgramPaused | Merge while paused |
| 10 | PrAlreadyProcessed | Duplicate PR merge |
| 11 | MilestoneCapExceeded | Escrow cap hit |
| 12 | InsufficientDeposit | Below `MIN_DEPOSIT` |
| 13 | InvalidRatio | Bad reward ratio |
| 14 | Overflow | Arithmetic overflow |
| 15 | ProgramNotPaused | Withdraw while active |

Unit tests in `test.rs` cover duplicate PR, milestone cap, and invalid repo cases.
