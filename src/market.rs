use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    pub rates: HashMap<String, f64>,
}

pub async fn fetch_rates() -> Result<ApiResponse, reqwest::Error> {
    let client = reqwest::Client::new();

    let response = client
        .get("https://open.er-api.com/v6/latest/USD")
        .header("User-Agent", "forex-bot")
        .send()
        .await?
        .text()
        .await?;

    let data: ApiResponse = serde_json::from_str(&response)
        .expect("Failed to parse JSON");

    Ok(data)
}