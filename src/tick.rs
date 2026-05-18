use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TickResponse {
    pub tick: Tick,
}

#[derive(Debug, Deserialize)]
pub struct Tick {
    pub quote: f64,
    pub bid: f64,
    pub ask: f64,
    pub epoch: i64,
    pub symbol: String,
}