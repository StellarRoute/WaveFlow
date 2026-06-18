#!/usr/bin/env bash
# Deploy WaveFlow escrow contract to Soroban testnet using the Soroban CLI.
set -euo pipefail

NETWORK="${SOROBAN_NETWORK:-testnet}"
RPC_URL="${SOROBAN_RPC_URL:-https://soroban-testnet.stellar.org}"

echo "Building waveflow-escrow WASM..."
cargo build --release -p waveflow-escrow --target wasm32-unknown-unknown

echo "Deploy with Soroban CLI (requires soroban config and funded identity):"
echo "  soroban contract deploy \\"
echo "    --wasm target/wasm32-unknown-unknown/release/waveflow_escrow.wasm \\"
echo "    --source-account <DEPLOYER_SECRET> \\"
echo "    --rpc-url ${RPC_URL} \\"
echo "    --network ${NETWORK}"
