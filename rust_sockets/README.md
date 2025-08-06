# Handy WebSockets for XRPL (Rust)

A lightweight Rust collection for monitoring XRP Ledger (XRPL) activity using WebSocket streams. This project provides small, runnable scripts to subscribe to various XRPL streams, such as ledgers, transactions, consensus phases, order book changes, validations, and account activity, using `tokio-tungstenite` for WebSocket communication. It mirrors the functionality of the JavaScript [`HandySockets`](https://github.com/HandyBoot/websockets) project.

## Overview

The `xrpl_rust` project includes scripts to connect to the XRPL via WebSocket (`wss://xrplcluster.com`) and log real-time updates for different streams. Each script is a standalone binary in `src/bin/`, designed to be simple and easy to run, making it ideal for developers exploring XRPL WebSocket APIs in Rust.

### Scripts and Their Purposes

- **`account_stream.rs`**: Tracks transactions affecting a specific account (`rMxCKbEDwqr76QuheSUMdEGf4B9xJ8m5De`).
- **`consensus_stream.rs`**: Tracks consensus phase changes (`open`, `establish`, `accepted`) during the XRPL consensus process.
- **`book_changes_stream.rs`**: Tracks order book changes across all trading pairs by subscribing to the `ledger` stream and requesting `book_changes`.
- **`ledger_stream.rs`**: Subscribes to the `ledger` stream to monitor validated ledgers, logging details like ledger index, hash, and transaction count.
- **`orderbook_stream.rs`**: Monitors the XRP/RLUSD order book for transaction updates.
- **`serverinfo_stream.rs`**: Retrieves server information for each validated ledger via the `ledger` stream and `server_info` command.
- **`transaction_stream.rs`**: Captures all validated transactions across the XRPL network.
- **`validation_stream.rs`**: Monitors validation messages (votes) from XRPL validators during consensus.

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

#### account_stream.rs
Tracks transactions for `rMxCKbEDwqr76QuheSUMdEGf4B9xJ8m5De`.

```json
{
  "close_time_iso": "2025-08-05T15:27:40Z",
  "ledger_index": 97957821,
  "hash": "848E9DD43593111500E681B636A188773AC6D934574B2F8CBAB963759FDC387B",
  "tx_json": {
    "TransactionType": "Payment",
    "Account": "rwtk4ZvffLedRFjTE2tyyTj6em58urK2UL",
    ...
  },
  "type": "transaction",
  "validated": true
}
```

#### bookchanges_stream.rs
Tracks order book changes across all trading pairs.

```json
{
  "ledger_index": 97957849,
  "ledger_hash": "589D0F6B0CC0FE57419BF9ADB9E91C774361B80803FEC8772D396D2A5362CC21",
  "changes": [
    {
      "currency_a": "XRP_drops",
      "currency_b": "rDgBV9WrwJ3WwtRWhkekMhDas3muFeKvoS/7372667800000000000000000000000000000000",
      "close": "8.437525048902489",
      "volume_a": "12000000",
      "volume_b": "1422218"
    },
    ...
  ],
  "type": "bookChanges",
  "validated": true
}
```

#### consensus_stream.rs
Tracks consensus phase changes.

```json
{
  "consensus": "open",
  "type": "consensusPhase"
}
{
  "consensus": "estabilsh",
  "type": "consensusPhase"
}
{
  "consensus": "accepted",
  "type": "consensusPhase"
}
```

#### ledger_stream.rs
Monitors validated ledgers.

```json
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
```

#### orderbook_stream.rs
Monitors XRP/RLUSD order book transactions.

```json
{
  "close_time_iso": "2025-08-05T15:28:30Z",
  "ledger_index": 97957834,
  "hash": "3CF6C5F0C3793B0AA849265452AB83CFA042BA7E16FFE162A5DE29BAC12008FF",
  "tx_json": {
    "TransactionType": "OfferCreate",
    "Account": "rpiFwLYi6Gb1ESHYorn2QG1WU5vw2u4exQ",
    "TakerGets": "640593690",
    "TakerPays": {
      "currency": "524C555344000000000000000000000000000000",
      "issuer": "rMxCKbEDwqr76QuheSUMdEGf4B9xJ8m5De",
      "value": "1912.90885"
    },
    ...
  },
  "type": "transaction",
  "validated": true
}
```

#### serverinfo_stream.rs
Retrieves server info for each validated ledger.

```json
{
  "info": {
    "build_version": "2.5.0",
    "server_state": "full",
    "validated_ledger": {
      "seq": 97957829,
      "hash": "29201F679D076F57AD6A264151DA839AD312BE8B9029DA4997CE433E59969BC3",
      ...
    },
    "uptime": 568402,
    "peers": 33,
    ...
  }
}
```

#### transaction_stream.rs
Monitors all validated transactions.

```json
{
  "close_time_iso": "2025-08-05T15:30:00Z",
  "ledger_index": 97957857,
  "hash": "934BCC18778C363E6E44BDA041A3F51E985AE2B92EAD02919DEEF9C89D1F0E68",
  "tx_json": {
    "TransactionType": "OfferCreate",
    "Account": "rfmdBKhtJw2J22rw1JxQcchQTM68qzE4N2",
    ...
  },
  "type": "transaction",
  "validated": true
}
```

#### validation_stream.rs
Captures validator votes.

```json
{
  "cookie": "3755884282517439284",
  "ledger_hash": "4828A52429B4805DEE51381FDDA7E89E1B76DA193A3B68239CBAFD75B3633B65",
  "ledger_index": 97957809,
  "type": "validationReceived",
  "validated_hash": "D80ECC86AE12132B1543395C0A30F9BC58184BF3FEE1C075FA0F740865722B81",
  ...
}
```



## Notes
- All scripts connect to the public XRPL WebSocket server `wss://xrplcluster.com`.
- Outputs are logged to the console in JSON format for easy parsing.
- Error handling is minimal; errors are logged via `eprintln!`.
- Each binary uses `tokio-tungstenite` for WebSocket communication and `serde_json` for JSON parsing.
- For detailed API documentation, see the [XRPL WebSocket API](https://xrpl.org/docs/references/http-websocket-apis/public-api-methods/subscription-methods/).

## Contributing
Feel free to fork the repository, add new stream monitors, or enhance existing scripts (e.g., with better error handling or output formatting). Submit pull requests to `https://github.com/Handy4ndy/HandySockets`.

## Author
Handy_4ndy
