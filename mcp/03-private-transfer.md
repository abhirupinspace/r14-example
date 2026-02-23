# MCP Walkthrough: Private Transfer

Transfer funds privately using ZK proofs via MCP tools.

**Prerequisite:** Complete `02-deposit-and-balance.md` (have a funded wallet).

## Step 1: Get Recipient's Owner Hash

The recipient runs `r14_keygen` and shares their `owner_hash` with you. Example:

```
recipient_hash = "0x7f8e9d..."
```

## Step 2: Transfer 300 to Recipient

**Tool:** `r14_transfer`

```
r14_transfer(recipient: "0x7f8e9d...", value: 300)
```

This will:
1. Select an unspent note with sufficient balance
2. Generate a Groth16 ZK proof
3. Submit the transfer on-chain

**Expected response:**
```json
{
  "nullifier": "0xdef456...",
  "out_commitment_0": "0x111...",
  "out_commitment_1": "0x222...",
  "tx_result": "SUCCESS",
  "message": "Transferred 300. Change note (700) created."
}
```

## Step 3: Verify Your Balance

**Tool:** `r14_balance`

```
r14_balance()
```

**Expected response:**
```json
{
  "total": 1200,
  "notes": [
    { "value": 1000, "spent": true },
    { "value": 500, "on_chain": true, "spent": false },
    { "value": 700, "on_chain": true, "spent": false }
  ]
}
```

The original 1000 note was consumed. You received 700 in change. Total: 500 + 700 = 1200.

## What Happened On-Chain

- A nullifier was published (proves the input note was spent, without revealing which one)
- Two new commitments were published (300 to recipient, 700 change to you)
- A ZK proof verified the transaction's validity
- No amounts, sender, or recipient are visible on-chain
