use tokio_tungstenite::{connect_async, tungstenite::Message};
use serde_json::{Value, json};
use futures_util::{SinkExt, StreamExt};

/*
Server info stream monitors server information for each validated ledger.
We subscribe to ledger stream and request server_info for each new validated ledger.

https://xrpl.org/docs/references/http-websocket-apis/public-api-methods/server-info-methods/server_info/
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

    // Prepare server_info request
    let server_info_request = json!({
        "command": "server_info"
    });

    // Listen for messages
    while let Some(msg) = read.next().await {
        let msg = msg?;
        if let Message::Text(text) = msg {
            let value: Value = serde_json::from_str(&text)?;
            if value.get("type").and_then(|t| t.as_str()) == Some("ledgerClosed") {
                // Print ledgerClosed event
                println!("New Validated Ledger:");
                println!("{}", serde_json::to_string_pretty(&value)?);
                println!("---");

                // Send server_info request
                write.send(Message::Text(serde_json::to_string(&server_info_request)?)).await?;
                println!("Fetching server info...");

                // Process server_info response
                if let Some(server_msg) = read.next().await {
                    let server_msg = server_msg?;
                    if let Message::Text(server_text) = server_msg {
                        let server_value: Value = serde_json::from_str(&server_text)?;
                        if server_value.get("result").is_some() && server_value["result"].get("info").is_some() {
                            println!("Server Info:");
                            println!("{}", serde_json::to_string_pretty(&server_value["result"]["info"])?);
                            println!("---");
                        }
                    }
                }
            }
        }
    }

    Ok(())
}