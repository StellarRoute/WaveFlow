# Gateway webhook HTTP responses

`github_webhook` maps internal errors to HTTP status codes via `map_error()` and `WaveFlowError::http_status_code()`.

## Success paths

| Outcome | HTTP | Body |
|---------|------|------|
| Payout recorded | 200 | `{ "status": "paid", ... }` |
| Ignored validation | 200 | `{ "status": "ignored", "reason": ... }` |
| Unsupported event | 200 | `{ "status": "ignored" }` |

Validation failures (for example non-merged PR) return 200 with ignored status so GitHub treats the delivery as accepted.

## Error paths

| Error type | HTTP |
|------------|------|
| `Unauthorized` (bad HMAC) | 401 |
| `Validation`, `Webhook` | 400 |
| `NotFound` | 404 |
| `Conflict` (duplicate PR) | 409 |
| `Database`, `Chain`, `Internal` | 500 |

Prometheus counters: `waveflow_gateway_webhook_rejected_total`, `waveflow_gateway_webhook_failed_total`, `waveflow_gateway_webhook_ignored_total`.

See `crates/shared/src/error.rs` and `crates/gateway/src/routes.rs`.
