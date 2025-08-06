use tokio_tungstenite::{connect_async, tungstenite::Message};
use serde_json::{Value, json};
use futures_util::{SinkExt, StreamExt};

/*
Book changes stream monitors order book changes across all trading pairs.
Since book_changes stream requires ledger index, we subscribe to ledger stream
and request book changes for each new validated ledger.

https://xrpl.org/docs/references/http-websocket-apis/public-api-methods/subscription-methods/subscribe#book-changes-stream
*/

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to XRPL WebSocket
    let (ws_stream, _) = connect_async("wss://xrplcluster.com").await?;
    println!("Connected to the XRPL!");

    let (mut write, mut read) = ws_stream.split();

    // Subscribe to ledger stream
    let subscribe_request = json!({
        "command": "subscribe",
        "streams": ["ledger"]
    });
    write.send(Message::Text(serde_json::to_string(&subscribe_request)?)).await?;
    println!("Listening for order book changes...");

    // Listen for ledgerClosed messages
    while let Some(msg) = read.next().await {
        let msg = msg?;
        if let Message::Text(text) = msg {
            let value: Value = serde_json::from_str(&text)?;
            if value.get("type").and_then(|t| t.as_str()) == Some("ledgerClosed") {
                // Extract ledger_index
                let ledger_index = value.get("ledger_index")
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| "Missing or invalid ledger_index")?;
                println!("Ledger {} validated, checking for book changes...", ledger_index);

                // Send book_changes request
                let book_changes_request = json!({
                    "command": "book_changes",
                    "ledger_index": ledger_index
                });
                write.send(Message::Text(serde_json::to_string(&book_changes_request)?)).await?;

                // Process book_changes response
                if let Some(book_msg) = read.next().await {
                    let book_msg = book_msg?;
                    if let Message::Text(book_text) = book_msg {
                        let book_value: Value = serde_json::from_str(&book_text)?;
                        if let Some(changes) = book_value.get("result").and_then(|r| r.get("changes")) {
                            if changes.as_array().map_or(false, |arr| !arr.is_empty()) {
                                println!("Order Book Changes:");
                                println!("{}", serde_json::to_string_pretty(&book_value["result"])?);
                                println!("---");
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(())
}