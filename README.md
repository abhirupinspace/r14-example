# r14-examples

Two Rust demos for [Root14](https://github.com/abhirupinspace/root-14-core) — a ZK privacy protocol for Stellar.

## Examples

| Example | Description | ZK Proofs |
|---------|-------------|-----------|
| `private-payments` | User A→User B deposit + transfer + balance | Groth16 (transfer circuit) |
| `zktls` | Web2 credentials → on-chain proof | Poseidon commitment + simulated range proof |

## Build & Run

```bash
cargo build --workspace
cargo run -p private-payments
cargo run -p zktls
```

Both examples run fully offline — no testnet or indexer required.

## private-payments

Full two-party private payment flow:
1. User A + User B keygen
2. User A deposits 1000
3. User A transfers 300 to User B with Groth16 proof
4. Final balances: User A=700, User B=300

## zktls

zkTLS concept — prove private web2 data on-chain without revealing it:
1. Mock TLS oracle fetches bank balance (15000)
2. Poseidon commitment hides the value
3. Range proof: "balance > 10000" (simulated — circuit not yet in r14-sdk)
4. Verification table shows what's proved vs revealed (nothing)

## Dependencies

Both examples use `r14-sdk` with the `prove` feature for ZK circuit access. First build may take a few minutes.
