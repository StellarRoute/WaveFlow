# Soroban storage keys reference

`DataKey` in `storage.rs` maps logical entities to Soroban persistent and instance storage.

## Instance keys (singleton)

| Key | Content |
|-----|---------|
| `Admin` | Maintainer `Address` |
| `Gateway` | Gateway `Address` |
| `Token` | Token contract `Address` |
| `ProgramCounter` | Monotonic id allocator |

## Persistent keys (parameterized)

| Key pattern | Content |
|-------------|---------|
| `Program(u64)` | `Program` struct (repo symbol, status, balances) |
| `Contributor(u64, Symbol)` | `Contributor` (Stellar address) |
| `ProcessedPr(u64, u64)` | `bool` idempotency flag |

Accessors `read_program`, `write_program`, `is_pr_processed`, and `mark_pr_processed` wrap env storage calls and return `ContractError` on missing data.

See `contracts/waveflow-escrow/src/storage.rs` and `types.rs`.
