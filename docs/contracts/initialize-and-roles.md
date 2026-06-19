# Contract initialize and roles

`initialize` in `waveflow-escrow` sets global contract state once.

## Arguments

- `admin`: maintainer address for pause/ratio/withdraw
- `gateway`: address allowed to call `record_merge`
- `token`: Soroban token contract for escrow deposits

## Storage keys written

`DataKey::Admin`, `Gateway`, `Token`, and `ProgramCounter` (starts at zero) persist in contract storage.

## Authorization model

- Admin-only: `pause`, `resume`, `set_ratio`, `withdraw_remaining`, `create_program`
- Gateway-only: `record_merge`
- Public read: program and contributor getters

Contract tests in `test.rs` use a `setup()` helper that calls `initialize` before each scenario.

See `contracts/waveflow-escrow/src/contract.rs` and `storage.rs`.
