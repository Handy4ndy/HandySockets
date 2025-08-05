use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use serde_json::{Value, json};
use tokio::main;
use futures_util::{SinkExt, StreamExt};

#[main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to XRPL WebSocket
    let (ws_stream, _) = connect_async("wss://xrplcluster.com")
        .await
        .map_err(|e| format!("Failed to connect to the XRPL: {}", e))?;
    println!("Connected to the XRPL!");

    let (mut write, mut read) = ws_stream.split();

    // Subscribe to XRP/RLUSD order book
    let subscribe_request = json!({
        "command": "subscribe",
        "books": [
            {
                "taker_pays": {
                    "currency": "XRP"
                },
                "taker_gets": {
                    "currency": "524C555344000000000000000000000000000000",
                    "issuer": "rMxCKbEDwqr76QuheSUMdEGf4B9xJ8m5De"
                },
                "snapshot": true
            }
        ]
    });
    write
        .send(Message::Text(serde_json::to_string(&subscribe_request)?))
        .await
        .map_err(|e| format!("Failed to subscribe to order book stream: {}", e))?;
    println!("Listening for XRP/RLUSD order book changes...");

    // Listen for transaction messages
    while let Some(msg) = read.next().await {
        let msg = msg.map_err(|e| format!("Failed to receive message: {}", e))?;
        if let Message::Text(text) = msg {
            let value: Value = serde_json::from_str(&text)?;
            if value.get("type").and_then(|t| t.as_str()) == Some("transaction") {
                println!("Order Book Transaction:");
                println!("{}", serde_json::to_string_pretty(&value)?);
                println!("---");
            }
        }
    }

    Ok(())
}