# webhook_events audit table

Every GitHub delivery processed by the gateway can leave a row in `webhook_events` for operator forensics.

## Schema (migration 001)

Columns include `delivery_id`, `event_type`, `action`, `program_id`, `payload` (JSONB), `status`, and `error_message`.

## Status values

| Status | When |
|--------|------|
| `processed` | Merge payout completed |
| `ignored` | Unsupported event type or validation skip |
| `failed` | Processing error before success |

Unsupported `x-github-event` values (not `pull_request`) persist with status `ignored` and HTTP 200 so GitHub does not retry endlessly.

## Persistence helper

`persist_webhook_event()` in `attestation.rs` is called from both success and ignored branches in `github_webhook`.

See `migrations/001_init.sql` and `crates/gateway/src/routes.rs`.
