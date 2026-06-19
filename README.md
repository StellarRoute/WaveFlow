# WaveFlow

Automated bounty escrow on Stellar/Soroban with GitHub merge-triggered payouts. WaveFlow brings Drips Wave-style mechanics onto the ledger: maintainers lock assets, contributors register Stellar wallets, and merged PRs trigger authorized attestations that pay rewards on-chain.

**Repository:** https://github.com/StellarRoute/WaveFlow

## Architecture

```
GitHub PR merge → Gateway (webhook + HMAC) → Soroban Escrow → Contributor wallet
                      ↓
                 Postgres audit trail
                      ↓
                 REST API (read paths)
```

## Crates

| Path | Role |
|------|------|
| `contracts/waveflow-escrow` | Soroban escrow, payout, and program admin logic |
| `crates/gateway` | GitHub webhook ingestion and chain attestation |
| `crates/api` | REST API for programs and payout history |
| `crates/shared` | Shared types, errors, and config |

## Prerequisites

- Rust stable (see `rust-toolchain.toml`)
- Docker (Postgres)
- Soroban CLI (for contract deploy)

## Quick start

```bash
cp .env.example .env
docker-compose up -d
cargo build --workspace
cargo test --workspace
```

Run gateway (webhooks):

```bash
cargo run -p waveflow-gateway
```

Run API:

```bash
cargo run -p waveflow-api
```

## Documentation

- [Product Requirements](docs/PRD.md)
- [Implementation Roadmap](docs/ROADMAP.md)
- [Core loop walkthrough](docs/core-loop.md)

## Production configuration

Set `SOROBAN_RPC_URL` and `NETWORK_PASSPHRASE` together when deploying the gateway:

| Network | `SOROBAN_RPC_URL` | `NETWORK_PASSPHRASE` |
|---------|-------------------|----------------------|
| Testnet | `https://soroban-testnet.stellar.org` | `Test SDF Network ; September 2015` |
| Mainnet | Your mainnet Soroban RPC endpoint | `Public Global Stellar Network ; September 2015` |

The gateway validates obvious testnet/mainnet mismatches at startup so production does not silently sign transactions for the wrong Stellar network.

## License

MIT
