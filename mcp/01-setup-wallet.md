# MCP Walkthrough: Setup Wallet

Generate a Root14 keypair and configure the wallet using MCP tools.

## Step 1: Generate Keypair

**Tool:** `r14_keygen`

```
r14_keygen()
```

**Expected response:**
```json
{
  "owner_hash": "0x1a2b3c...",
  "wallet_path": "~/.r14/wallet.json",
  "message": "Keypair generated. Configure stellar_secret and contract IDs next."
}
```

## Step 2: Configure Stellar Secret

**Tool:** `r14_config_set`

```
r14_config_set(key: "stellar_secret", value: "SXXX...your_testnet_secret")
```

## Step 3: Configure Core Contract

**Tool:** `r14_config_set`

```
r14_config_set(key: "core_contract_id", value: "CXXX...your_core_contract")
```

## Step 4: Configure Transfer Contract

**Tool:** `r14_config_set`

```
r14_config_set(key: "transfer_contract_id", value: "CXXX...your_transfer_contract")
```

## Step 5: Verify Config

**Tool:** `r14_config_show`

```
r14_config_show()
```

**Expected response:**
```json
{
  "owner_hash": "0x1a2b3c...",
  "stellar_secret": "SXXX...***masked***",
  "core_contract_id": "CXXX...",
  "transfer_contract_id": "CXXX...",
  "indexer_url": "https://...",
  "rpc_url": "https://..."
}
```

## Step 6: Check Health

**Tool:** `r14_status`

```
r14_status()
```

All services should report healthy. You're ready to deposit.
