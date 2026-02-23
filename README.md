# r14-examples

Two Rust demos for [Root14](https://github.com/abhirupinspace/root-14-core) — a ZK privacy protocol for Stellar.

## Examples

| Example | Description | ZK Proofs | Network |
|---------|-------------|-----------|---------|
| `private-payments` | User A→User B deposit + transfer + balance | Groth16 (transfer circuit) | Offline |
| `zktls` | Web2 credentials → testnet deposit → private ZK transfer | Poseidon commitment + Groth16 | Stellar testnet |

## Build & Run

```bash
cargo build --workspace
cargo run -p private-payments
cargo run -p zktls
```

`private-payments` runs fully offline. `zktls` requires a configured wallet and Stellar testnet access.

## private-payments

Full two-party private payment flow (offline):
1. User A + User B keygen
2. User A deposits 1000
3. User A transfers 300 to User B with Groth16 proof
4. Final balances: User A=700, User B=300

## zktls

End-to-end zkTLS flow with real Stellar testnet transactions:
1. Load wallet from `~/.r14/wallet.json`
2. TLS oracle fetches private bank balance (simulated source, real value)
3. Poseidon commitment hides the value
4. Deposit the value as a shielded note on Stellar testnet
5. Balance check confirms on-chain note
6. Private transfer to a second user with Groth16 ZK proof
7. Final balance reflects the transfer — value never revealed on-chain

**Setup:** Run `r14_keygen` then configure `stellar_secret`, `core_contract_id`, and `transfer_contract_id` via `r14_config_set`.

## Dependencies

Both examples use `r14-sdk` with the `prove` feature for ZK circuit access. First build may take a few minutes.
