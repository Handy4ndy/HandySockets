# Handy WebSockets for XRPL (Rust)

A lightweight Rust library for monitoring XRP Ledger (XRPL) activity using WebSocket streams. This project provides small, runnable scripts to subscribe to various XRPL streams, such as ledgers, transactions, consensus phases, order book changes, validations, and account activity, using `tokio-tungstenite` for WebSocket communication. It mirrors the functionality of the JavaScript [`handy_websockets`](https://github.com/HandyBoot/websockets) project.

## Overview

The `xrpl_rust` project includes scripts to connect to the XRPL via WebSocket (`wss://xrplcluster.com`) and log real-time updates for different streams. Each script is a standalone binary in `src/bin/`, designed to be simple and easy to run, making it ideal for developers exploring XRPL WebSocket APIs in Rust.

### Scripts and Their Purposes

- **`ledger_stream.rs`**: Subscribes to the `ledger` stream to monitor validated ledgers, logging details like ledger index, hash, and transaction count.
- **`consensus_stream.rs`**: Tracks consensus phase changes (`open`, `establish`, `accepted`) during the XRPL consensus process.
- **`validation_stream.rs`**: Monitors validation messages (votes) from XRPL validators during consensus.
- **`transaction_stream.rs`**: Captures all validated transactions across the XRPL network.
- **`account_stream.rs`**: Tracks transactions affecting a specific account (`rMxCKbEDwqr76QuheSUMdEGf4B9xJ8m5De`).
- **`orderbook_stream.rs`**: Monitors the XRP/RLUSD order book for transaction updates.
- **`book_changes_stream.rs`**: Tracks order book changes across all trading pairs by subscribing to the `ledger` stream and requesting `book_changes`.
- **`serverinfo_stream.rs`**: Retrieves server information for each validated ledger via the `ledger` stream and `server_info` command.

## Prerequisites

- **Rust**: Version 1.80 or higher. Install via [rustup](https://rustup.rs/):
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **OpenSSL**: Required for `tokio-tungstenite` with `native-tls`. On Ubuntu:
  ```bash
  sudo apt update
  sudo apt install -y libssl-dev
  ```

## Installation

1. Clone or download the repository:
   ```bash
   git clone https://github.com/HandySockets/xrpl_rust.git
   cd xrpl_rust
   ```
2. Build the project:
   ```bash
   cargo build
   ```

## Usage

Each script is a separate binary and can be run independently with `cargo run --bin <script-name>`. Ensure you’re in the project directory (`~/Documents/GitHub/HandySockets/xrpl_rust`).

```bash
cargo run --bin <script-name>
```

For example:
```bash
cargo run --bin ledger_stream
```

### Example Outputs

Below are sample outputs from running the scripts, showing the type of data each captures.

#### ledger_stream.rs
Monitors validated ledgers.

```json
Connected to the XRPL!
Listening for validated ledgers...
New Validated Ledger:
{
  "fee_base": 10,
  "ledger_hash": "147AAE9D261F004674454AAABA484FDF930C83154E2EE0CE7416AD5BA1CC3A7E",
  "ledger_index": 97961142,
  "ledger_time": 807722931,
  "reserve_base": 1000000,
  "reserve_inc": 200000,
  "txn_count": 131,
  "type": "ledgerClosed",
  "validated_ledgers": "32570-97961142"
}
---
```

#### consensus_stream.rs
Tracks consensus phase changes.

```json
Connected to the XRPL!
Listening for consensus phase changes...
Consensus Phase Change:
{
  "consensus": "accepted",
  "type": "consensusPhase"
}
---
Consensus Phase Change:
{
  "consensus": "open",
  "type": "consensusPhase"
}
---
```

#### validation_stream.rs
Captures validator votes.

```json
Connected to the XRPL!
Listening for validation events...
Validation Received:
{
  "type": "validationReceived",
  "ledger_index": "97961142",
  "ledger_hash": "147AAE9D261F004674454AAABA484FDF930C83154E2EE0CE7416AD5BA1CC3A7E",
  ...
}
---
```

#### transaction_stream.rs
Monitors all validated transactions.

```json
Connected to the XRPL!
Listening for transaction events...
New Transaction Event:
{
  "type": "transaction",
  "transaction": {
    "TransactionType": "OfferCreate",
    "Account": "rfmdBKhtJw2J22rw1JxQcchQTM68qzE4N2",
    ...
  },
  "ledger_index": 97961142,
  "validated": true
}
---
```

#### account_stream.rs
Tracks transactions for `rMxCKbEDwqr76QuheSUMdEGf4B9xJ8m5De`.

```json
Connected to the XRPL!
Listening for transactions on account: rMxCKbEDwqr76QuheSUMdEGf4B9xJ8m5De
New Transaction:
{
  "type": "transaction",
  "transaction": {
    "TransactionType": "Payment",
    "Account": "rMxCKbEDwqr76QuheSUMdEGf4B9xJ8m5De",
    ...
  },
  "ledger_index": 97961142,
  "validated": true
}
---
```

#### orderbook_stream.rs
Monitors XRP/RLUSD order book transactions.

```json
Connected to the XRPL!
Listening for XRP/RLUSD order book changes...
Order Book Transaction:
{
  "type": "transaction",
  "transaction": {
    "TransactionType": "OfferCreate",
    "Account": "rpiFwLYi6Gb1ESHYorn2QG1WU5vw2u4exQ",
    "TakerGets": "640593690",
    "TakerPays": {
      "currency": "524C555344000000000000000000000000000000",
      "issuer": "rMxCKbEDwqr76QuheSUMdEGf4B9