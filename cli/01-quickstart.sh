#!/usr/bin/env bash
# Root14 CLI Quickstart — keygen → config → deposit → balance
# Prerequisites: r14 CLI installed, Stellar testnet account funded
set -euo pipefail

echo "=== 1. Generate Keypair ==="
r14 keygen

echo ""
echo "=== 2. Configure Wallet ==="
# Set your Stellar secret key (testnet)
r14 config set stellar_secret "${STELLAR_SECRET:?Set STELLAR_SECRET env var}"
r14 config set core_contract_id "${CORE_CONTRACT_ID:?Set CORE_CONTRACT_ID env var}"
r14 config set transfer_contract_id "${TRANSFER_CONTRACT_ID:?Set TRANSFER_CONTRACT_ID env var}"

echo ""
echo "=== 3. Verify Config ==="
r14 config show

echo ""
echo "=== 4. Deposit 1000 ==="
r14 deposit 1000

echo ""
echo "=== 5. Check Balance ==="
r14 balance

echo ""
echo "Done! You now have a shielded note worth 1000."
