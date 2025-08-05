use tokio_tungstenite::{connect_async, tungstenite::Message};
use serde_json::{Value, json};
use futures_util::{SinkExt, StreamExt};

/*
Consensus stream monitors consensus phase changes during the XRPL consensus process.
Sends messages when the server changes phase in the consensus cycle.

https://xrpl.org/docs/references/http-websocket-apis/public-api-methods/subscription-methods/subscribe#consensus-stream
*/

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (ws_stream, _) = connect_async("wss://xrplcluster.com").await?;
    println!("Connected to the XRPL!");

    let (mut write, mut read) = ws_stream.split();

    let subscribe_request = json!({
        "command": "subscribe",
        "streams": ["consensus"]
    });
    write.send(Message::Text(serde_json::to_string(&subscribe_request)?)).await?;
    println!("Listening for consensus phase changes...");

    while let Some(msg) = read.next().await {
        let msg = msg?;
        if let Message::Text(text) = msg {
            let value: Value = serde_json::from_str(&text)?;
            if value.get("type").and_then(|t| t.as_str()) == Some("consensusPhase") {
                println!("Consensus Phase Change:");
                println!("{}", serde_json::to_string_pretty(&value)?);
                println!("---");
            }
        }
    }

    Ok(())
}