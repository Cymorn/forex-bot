use tokio_tungstenite::connect_async;
use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use crate::state::AppState;

pub async fn start_market(state: AppState) {
    let url = "wss://ws.derivws.com/websockets/v3?app_id=1089";

    let (ws_stream, _) = connect_async(url)
        .await
        .expect("Failed to connect");

    let (mut write, mut read) = ws_stream.split();

    let subscribe = json!({
        "ticks": "frxEURUSD",
        "subscribe": 1
    });

    write.send(
        tokio_tungstenite::tungstenite::Message::Text(subscribe.to_string().into())
    )
    .await
    .unwrap();

    println!("Market stream started");

    while let Some(msg) = read.next().await {
        if let Ok(m) = msg {
            let text = m.to_text().unwrap_or("");

            if let Ok(v) = serde_json::from_str::<serde_json::Value>(text) {
                if let Some(price) = v["tick"]["quote"].as_f64() {
                    state.update_price(price);
                }
            }
        }
    }
}