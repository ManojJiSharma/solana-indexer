use tokio::sync::mpsc::Sender;

use futures_util::{SinkExt, StreamExt};
use serde_json::{Value, json};
use tokio_tungstenite::connect_async;
use url::Url;

#[derive(Debug)]
pub struct LogEvent {
    pub signature: String,
    pub slot: u64,
    pub logs: Vec<String>,
}

pub async fn connect_rpc(ws_url: &str, sender: Sender<LogEvent>) -> anyhow::Result<()> {
    let url = ws_url;

    Url::parse(url)?;

    let (ws_stream, _response) = connect_async(url).await?;

    println!("Connected to Solana Devnet WebSocket");

    let (mut write, mut read) = ws_stream.split();

    let subscribe_msg = json!({
      "jsonrpc": "2.0",
      "id": 1,
      "method": "logsSubscribe",
      "params": [
        "all",
        { "commitment": "finalized" }
      ]
    });

    write.send(subscribe_msg.to_string().into()).await?;

    println!("Subscribed to transaction logs");

    while let Some(msg) = read.next().await {
        let msg = match msg {
            Ok(m) => m,
            Err(e) => {
                eprintln!("error in msg : {}", e);
                break;
            }
        };

        let text = match msg.to_text() {
            Ok(t) => t,
            Err(_) => continue,
        };

        let v: Value = match serde_json::from_str(text) {
            Ok(val) => val,
            Err(_) => continue,
        };

        // Skip subscription confirmation messages
        if v.get("result").is_some() && v.get("id").is_some() {
            continue;
        }

        // Only process notification messages
        if v.get("params").is_none() {
            continue;
        }

        let result = &v["params"]["result"];

        let signature = result["value"]["signature"]
            .as_str()
            .unwrap_or_default()
            .to_string();

        let slot = result["context"]["slot"].as_u64().unwrap_or(0);

        let logs = result["value"]["logs"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|l| l.as_str().map(String::from))
            .collect::<Vec<_>>();

        if signature.is_empty() {
            continue;
        }

        let log_event = LogEvent {
            signature,
            slot,
            logs,
        };

        if sender.send(log_event).await.is_err() {
            break;
        }
    }

    Ok(())
}