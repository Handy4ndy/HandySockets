use tokio_tungstenite::{connect_async, tungstenite::Message};
use serde_json::{Value, json};
use futures_util::{SinkExt, StreamExt};

/*
The ledger stream only sends ledgerClosed messages when the consensus process declares a new
validated ledger. The message identifies the ledger and provides some information about its
contents. 

https://xrpl.org/docs/references/http-websocket-apis/public-api-methods/subscription-methods/subscribe#ledger-stream
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
    println!("Listening for validated ledgers...");

    // Listen for ledgerClosed messages
    while let Some(msg) = read.next().await {
        let msg = msg?;
        if let Message::Text(text) = msg {
            let value: Value = serde_json::from_str(&text)?;
            if value.get("type").and_then(|t| t.as_str()) == Some("ledgerClosed") {
                println!("New Validated Ledger:");
                println!("{}", serde_json::to_string_pretty(&value)?);
                println!("---");
            }
        }
    }

    Ok(())
}