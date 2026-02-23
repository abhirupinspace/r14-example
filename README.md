# r14-examples

Example projects for [Root14](https://github.com/abhirupinspace/root-14-core) — a ZK privacy protocol for Stellar.

## Examples

### Rust (`rust/`)

| Example | Description | ZK Proofs |
|---------|-------------|-----------|
| `01-basic-keygen` | Generate keypair, print owner hash | No |
| `02-offline-notes` | Create notes, commitments, merkle tree | No |
| `03-private-payment` | Full deposit → transfer → balance flow | Yes |
| `04-two-party` | Alice → Bob private transfer demo | Yes |

### CLI (`cli/`)

Shell scripts for the `r14` command-line tool.

| Script | Description |
|--------|-------------|
| `01-quickstart.sh` | Keygen → config → deposit → balance |
| `02-transfer.sh` | Private transfer between wallets |
| `03-status-check.sh` | Status and config inspection |

### MCP (`mcp/`)

Walkthroughs for Root14 MCP tools (Claude Code / MCP clients).

| Guide | Description |
|-------|-------------|
| `01-setup-wallet.md` | Keygen → config via MCP |
| `02-deposit-and-balance.md` | Deposit + balance via MCP |
| `03-private-transfer.md` | Full transfer via MCP |

## Quickstart

```bash
# run the simplest example (no network needed)
cargo run -p basic-keygen

# run offline note/merkle demo
cargo run -p offline-notes

# run full payment demo (offline by default, set env vars for testnet)
cargo run -p private-payment

# run Alice→Bob demo
cargo run -p two-party
```

## Testnet Configuration

For live testnet examples, set:

```bash
export R14_STELLAR_SECRET="S..."
export R14_CORE_CONTRACT="C..."
export R14_TRANSFER_CONTRACT="C..."
export R14_INDEXER_URL="https://..."
```

## Building

```bash
cargo build --workspace
```

Examples 03 and 04 require the `prove` feature (enabled by default in their Cargo.toml) which pulls in the ZK circuit. First build may take a few minutes.
