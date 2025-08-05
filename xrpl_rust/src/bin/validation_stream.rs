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

    // Subscribe to validations stream
    let subscribe_request = json!({
        "command": "subscribe",
        "streams": ["validations"]
    });
    write
        .send(Message::Text(serde_json::to_string(&subscribe_request)?))
        .await
        .map_err(|e| format!("Failed to subscribe to validations stream: {}", e))?;
    println!("Listening for validation events...");

    // Listen for validationReceived messages
    while let Some(msg) = read.next().await {
        let msg = msg.map_err(|e| format!("Failed to receive message: {}", e))?;
        if let Message::Text(text) = msg {
            let value: Value = serde_json::from_str(&text)?;
            if value.get("type").and_then(|t| t.as_str()) == Some("validationReceived") {
                println!("Validation Received:");
                println!("{}", serde_json::to_string_pretty(&value)?);
                println!("---");
            }
        }
    }

    Ok(())
}