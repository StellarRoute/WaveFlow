# Program lifecycle admin operations

Maintainers control escrow programs through admin-only contract functions.

## pause / resume

`pause` sets `ProgramStatus::Paused`. While paused, `record_merge` returns `ProgramPaused` (code 9). `resume` restores active status.

## set_ratio

Updates `reward_per_point` for an existing program. Invalid values return `InvalidRatio` (code 13).

## withdraw_remaining

Admin withdraws leftover escrow after program completion. Requires program to be paused first (`ProgramNotPaused` otherwise).

## Test coverage

`paused_program_rejects_merge` in `test.rs` asserts merge failure on paused programs.

See `contracts/waveflow-escrow/src/contract.rs` and `types.rs` (`ProgramStatus`).
