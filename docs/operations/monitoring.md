# Operations monitoring

## Health endpoints

| Service | URL | Checks |
|---------|-----|--------|
| API | `GET /health` | Postgres `SELECT 1` |
| Gateway | `GET /health` | Service liveness |

Render uses these paths from `render.yaml` healthCheckPath.

## Metrics

Gateway and API increment Prometheus-style counters via the `metrics` crate (e.g. `waveflow_api_health_checks_total` in API health handler).

Configure `OTEL_EXPORTER_OTLP_ENDPOINT` for distributed tracing export.

## Logging

Set `RUST_LOG` per `.env.example`:

```
RUST_LOG=info,waveflow_gateway=debug,waveflow_api=debug
```

## Audit reconciliation

Compare Postgres `payouts.tx_hash` rows against Soroban transaction history for a program. Mismatches indicate gateway retry or idempotency issues.

## Alerts (recommended)

- Gateway webhook 5xx rate
- Payout rows with null `tx_hash` after threshold
- Postgres connection failures on `/health`
