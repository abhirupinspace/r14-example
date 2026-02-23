#!/usr/bin/env bash
# Root14 CLI Transfer — private transfer between two wallets
# Prerequisites: run 01-quickstart.sh first to have a funded wallet
set -euo pipefail

RECIPIENT_HASH="${1:?Usage: ./02-transfer.sh <recipient_owner_hash>}"

echo "=== 1. Check Current Balance ==="
r14 balance

echo ""
echo "=== 2. Transfer 300 to Recipient ==="
echo "recipient: $RECIPIENT_HASH"
r14 transfer "$RECIPIENT_HASH" 300

echo ""
echo "=== 3. Check Updated Balance ==="
r14 balance

echo ""
echo "Done! Transferred 300 privately to $RECIPIENT_HASH."
echo "The recipient can run 'r14 balance' to see the received note."
