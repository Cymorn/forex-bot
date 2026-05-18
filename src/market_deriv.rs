use tokio_tungstenite::connect_async;
use futures_util::{SinkExt, StreamExt};
use serde_json::json;

pub async fn connect_deriv() {
    let url = "wss://ws.derivws.com/websockets/v3?app_id=1089";

    let (ws_stream, _) = connect_async(url)
        .await
        .expect("Failed to connect to Deriv");

    println!("Connected to Deriv");

    let (mut write, mut read) = ws_stream.split();

    // Subscribe to EUR/USD ticks
    let subscribe = json!({
        "ticks": "frxEURUSD",
        "subscribe": 1
    });

    write.send(
        tokio_tungstenite::tungstenite::Message::Text(subscribe.to_string().into())
    )
    .await
    .unwrap();

    while let Some(msg) = read.next().await {
        match msg {
            Ok(m) => {
                println!("Market Data: {:?}", m);
            }
            Err(e) => {
                println!("Error: {:?}", e);
                break;
            }
        }
    }
}