use tokio_tungstenite::{connect_async, tungstenite::Message};
use serde_json::{Value, json};
use futures_util::{SinkExt, StreamExt};

/*
Account stream monitors a specific account for validated transactions.
Sends transaction messages whenever a transaction affects the monitored account.
Subscribed to the RLUSD account to receive updates.

https://xrpl.org/docs/references/http-websocket-apis/public-api-methods/subscription-methods/subscribe#accounts
*/

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to XRPL WebSocket
    let (ws_stream, _) = connect_async("wss://xrplcluster.com").await?;
    println!("Connected to the XRPL!");

    let (mut write, mut read) = ws_stream.split();

    // Subscribe to account stream for RLUSD account
    let subscribe_request = json!({
        "command": "subscribe",
        "accounts": ["rMxCKbEDwqr76QuheSUMdEGf4B9xJ8m5De"]
    });
    write.send(Message::Text(serde_json::to_string(&subscribe_request)?)).await?;
    println!("Listening for transactions on account: rMxCKbEDwqr76QuheSUMdEGf4B9xJ8m5De");

    // Listen for transaction messages
    while let Some(msg) = read.next().await {
        let msg = msg?;
        if let Message::Text(text) = msg {
            let value: Value = serde_json::from_str(&text)?;
            if value.get("type").and_then(|t| t.as_str()) == Some("transaction") {
                println!("New Transaction:");
                println!("{}", serde_json::to_string_pretty(&value)?);
                println!("---");
            }
        }
    }

    Ok(())
}