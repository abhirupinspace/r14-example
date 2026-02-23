#!/usr/bin/env bash
# Root14 CLI Status — check system health and wallet config
set -euo pipefail

echo "=== 1. System Status ==="
r14 status

echo ""
echo "=== 2. Wallet Config ==="
r14 config show

echo ""
echo "=== 3. Current Balance ==="
r14 balance
