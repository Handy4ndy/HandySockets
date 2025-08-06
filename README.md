# HandySockets: XRPL WebSocket Collection

A collection of lightweight JavaScript and Rust scripts for monitoring XRP Ledger (XRPL) activity using WebSocket streams. This project includes two subdirectories, `js_sockets` (JavaScript) and `rust_sockets` (Rust), each containing scripts that connect to the XRPL via WebSocket (`wss://xrplcluster.com`) to stream real-time data for various XRPL events, such as ledgers, consensus phases, transactions, and order book changes.

## Overview

The `HandySockets` collection provides simple, runnable scripts to interact with XRPL WebSocket APIs, ideal for developers exploring real-time XRPL monitoring. The JavaScript scripts use the `xrpl` library, while the Rust scripts use `tokio-tungstenite` for reliable WebSocket communication. Both projects subscribe to XRPL streams and commands, as defined in the [XRPL WebSocket API](https://xrpl.org/docs/references/http-websocket-apis/public-api-methods/subscription-methods/), including:

- **[Accounts Stream](https://xrpl.org/docs/references/http-websocket-apis/public-api-methods/subscription-methods/subscribe#accounts)**: Tracks transactions affecting a specific account (e.g., `rMxCKbEDwqr76QuheSUMdEGf4B9xJ8m5De`).
- **[Consensus Stream](https://xrpl.org/docs/references/http-websocket-apis/public-api-methods/subscription-methods/subscribe#consensus-stream)**: Tracks consensus phase changes (`open`, `establish`, `accepted`) during the XRPL consensus process.
- **[Book Changes](https://xrpl.org/docs/references/http-websocket-apis/public-api-methods/subscription-methods/subscribe#book-changes-stream)**: Tracks order book changes across all trading pairs using the `ledger` stream and `book_changes` command.
- **[Ledger Stream](https://xrpl.org/docs/references/http-websocket-apis/public-api-methods/subscription-methods/subscribe#ledger-stream)**: Monitors validated ledgers, providing details like ledger index, hash, and transaction count.
- **[Order Book Stream](https://xrpl.org/docs/references/http-websocket-apis/public-api-methods/subscription-methods/subscribe#order-book-streams)**: Monitors transactions affecting the XRP/RLUSD order book.
- **[Server Info](https://xrpl.org/docs/references/http-websocket-apis/public-api-methods/server-info-methods)**: Retrieves server state for each validated ledger.
- **[Transactions Stream](https://xrpl.org/docs/references/http-websocket-apis/public-api-methods/subscription-methods/subscribe#transaction-streams)**: Monitors all validated transactions across the XRPL network.
- **[Validations Stream](https://xrpl.org/docs/references/http-websocket-apis/public-api-methods/subscription-methods/subscribe#validations-stream)**: Captures validation messages (votes) from XRPL validators.

## Projects and Scripts

### js_sockets (JavaScript)
- **`accountStream.js`**: Subscribes to the `accounts` stream for `rMxCKbEDwqr76QuheSUMdEGf4B9xJ8m5De`.
- **`bookchangesStream.js`**: Subscribes to the `ledger` stream and requests `book_changes`.
- **`consensusStream.js`**: Subscribes to the `consensus` stream for phase changes.
- **`ledgerStream.js`**: Subscribes to the `ledger` stream, logs validated ledgers.
- **`orderbookStream.js`**: Subscribes to the XRP/RLUSD order book stream.
- **`serverinfoStream.js`**: Subscribes to the `ledger` stream and requests `server_info`.
- **`transactionStream.js`**: Subscribes to the `transactions` stream for all transactions.
- **`validationStream.js`**: Subscribes to the `validations` stream for validator votes.

### rust_sockets (Rust)
- **`account_stream.rs`**: Subscribes to the `accounts` stream for `rMxCKbEDwqr76QuheSUMdEGf4B9xJ8m5De`.
- **`book_changes_stream.rs`**: Subscribes to the `ledger` stream and requests `book_changes`.
- **`consensus_stream.rs`**: Subscribes to the `consensus` stream for phase changes.
- **`ledger_stream.rs`**: Subscribes to the `ledger` stream, logs validated ledgers.
- **`orderbook_stream.rs`**: Subscribes to the XRP/RLUSD order book stream.
- **`serverinfoStream.rs`**: Subscribes to the `ledger` stream and requests `server_info`.
- **`transaction_stream.rs`**: Subscribes to the `transactions` stream for all transactions.
- **`validation_stream.rs`**: Subscribes to the `validations` stream for validator votes.

## Prerequisites

- **JavaScript (js_sockets)**:
  - Node.js (v14 or higher): [Download](https://nodejs.org/).
  - `xrpl` library (v4.4.0 or higher).

- **Rust (rust_sockets)**:
  - Rust (v1.80 or higher): Install via [rustup](https://rustup.rs/):
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```
  - OpenSSL: On Ubuntu:
    ```bash
    sudo apt update
    sudo apt install -y libssl-dev
    ```

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/Handy4ndy/HandySockets.git
   cd HandySockets
   ```

2. **JavaScript (js_sockets)**:
   ```bash
   cd js_sockets
   npm install xrpl
   ```

3. **Rust (rust_sockets)**:
   ```bash
   cd rust_sockets
   cargo build
   ```

## Usage

Run scripts from their respective directories.

- **JavaScript**:
  ```bash
  cd js_sockets
  node <script-name>.js
  ```
  Example:
  ```bash
  node ledgerStream.js
  ```

- **Rust**:
  ```bash
  cd rust_sockets
  cargo run --bin <script-name>
  ```
  Example:
  ```bash
  cargo run --bin ledger_stream
  ```

## Notes
- All scripts connect to the public XRPL WebSocket server `wss://xrplcluster.com`.
- Outputs are logged to the console in JSON format.
- Error handling is minimal; errors are logged with descriptive messages.
- For detailed XRPL WebSocket method documentation, visit [xrpl.org](https://xrpl.org/docs/references/http-websocket-apis/).

## Contributing
Fork the repository, add new scripts, or enhance existing ones in `js_sockets` or `rust_sockets`. Submit pull requests to `https://github.com/Handy4ndy/HandySockets`.

## Author
Handy_4ndy

## License
MIT License.
