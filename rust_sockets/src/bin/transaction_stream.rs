use tokio_tungstenite::{connect_async, tungstenite::Message};
use serde_json::{Value, json};
use futures_util::{SinkExt, StreamExt};

/*
Transaction stream monitors all validated transactions on the XRPL network.
Sends transaction messages whenever any transaction is included in a validated ledger.

https://xrpl.org/docs/references/http-websocket-apis/public-api-methods/subscription-methods/subscribe#transaction-streams
*/

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (ws_stream, _) = connect_async("wss://xrplcluster.com").await?;
    println!("Connected to the XRPL!");

    let (mut write, mut read) = ws_stream.split();

    let subscribe_request = json!({
        "command": "subscribe",
        "streams": ["transactions"]
    });
    write.send(Message::Text(serde_json::to_string(&subscribe_request)?)).await?;
    println!("Listening for transaction events...");

    while let Some(msg) = read.next().await {
        let msg = msg?;
        if let Message::Text(text) = msg {
            let value: Value = serde_json::from_str(&text)?;
            if value.get("type").and_then(|t| t.as_str()) == Some("transaction") {
                println!("New Transaction Event:");
                println!("{}", serde_json::to_string_pretty(&value)?);
                println!("---");
            }
        }
    }

    Ok(())
}