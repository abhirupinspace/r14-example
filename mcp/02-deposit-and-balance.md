# MCP Walkthrough: Deposit and Balance

Shield funds and check your balance using MCP tools.

**Prerequisite:** Complete `01-setup-wallet.md` first.

## Step 1: Deposit 1000

**Tool:** `r14_deposit`

```
r14_deposit(value: 1000)
```

**Expected response:**
```json
{
  "commitment": "0xabc123...",
  "value": 1000,
  "tx_result": "SUCCESS",
  "message": "Deposited 1000. Note created and stored in wallet."
}
```

This creates a shielded note worth 1000 on-chain. The note is stored locally in your wallet.

## Step 2: Check Balance

**Tool:** `r14_balance`

```
r14_balance()
```

**Expected response:**
```json
{
  "total": 1000,
  "notes": [
    {
      "value": 1000,
      "commitment": "0xabc123...",
      "on_chain": true,
      "spent": false
    }
  ]
}
```

## Step 3: Make Another Deposit

```
r14_deposit(value: 500)
```

## Step 4: Verify Updated Balance

```
r14_balance()
```

**Expected response:**
```json
{
  "total": 1500,
  "notes": [
    { "value": 1000, "on_chain": true, "spent": false },
    { "value": 500, "on_chain": true, "spent": false }
  ]
}
```

Your shielded balance is now 1500 across two notes. Ready for transfers.
