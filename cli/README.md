# Root14 CLI Examples

Shell scripts demonstrating the `r14` CLI tool.

## Prerequisites

- `r14` CLI installed (`cargo install r14-cli` or build from source)
- Stellar testnet account with funds
- Environment variables set:
  - `STELLAR_SECRET` — your Stellar secret key
  - `CORE_CONTRACT_ID` — deployed core contract address
  - `TRANSFER_CONTRACT_ID` — deployed transfer contract address

## Scripts

| Script | Description |
|--------|-------------|
| `01-quickstart.sh` | Keygen, config, deposit, balance |
| `02-transfer.sh` | Private transfer to a recipient |
| `03-status-check.sh` | System health and config inspection |

## Usage

```bash
# set env vars
export STELLAR_SECRET="S..."
export CORE_CONTRACT_ID="C..."
export TRANSFER_CONTRACT_ID="C..."

# run quickstart
bash cli/01-quickstart.sh

# transfer to someone (pass their owner_hash)
bash cli/02-transfer.sh 0xabc123...

# check status
bash cli/03-status-check.sh
```
