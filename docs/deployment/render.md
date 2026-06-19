# Render deployment

Blueprint file: `render.yaml`

## Services

| Render service | Dockerfile | Port | Health |
|----------------|------------|------|--------|
| `waveflow-gateway` | `Dockerfile.gateway` | 8080 (`PORT`) | `/health` |
| `waveflow-api` | `Dockerfile.api` | 8081 | `/health` |
| `waveflow-db` | managed Postgres | n/a | n/a |

## Required secrets (sync: false in blueprint)

- `GITHUB_WEBHOOK_SECRET` on gateway
- `ESCROW_CONTRACT_ID` on gateway
- `GATEWAY_SECRET_KEY` on gateway
- `API_ADMIN_KEYS` on API

## GitHub webhook URL

Point repository webhook to:

```
https://<waveflow-gateway-host>/webhooks/github
```

Events: **Pull requests**. Content type: `application/json`.

## Database

Both services receive `DATABASE_URL` from the Render Postgres attachment.

## Soroban

Default blueprint sets `SOROBAN_RPC_URL` and `NETWORK_PASSPHRASE` to testnet:

| Network | `SOROBAN_RPC_URL` | `NETWORK_PASSPHRASE` |
|---------|-------------------|----------------------|
| Testnet | `https://soroban-testnet.stellar.org` | `Test SDF Network ; September 2015` |
| Mainnet | Your mainnet Soroban RPC endpoint | `Public Global Stellar Network ; September 2015` |

Override both values together when moving the gateway to mainnet. The gateway rejects obvious testnet/mainnet RPC and passphrase mismatches during startup.

## Local parity

Compare with [docker-compose.md](docker-compose.md) for local Postgres and port mapping differences (local API on 8081, gateway on 8080).
