use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Ohlcv {
    pub timestamp: DateTime<Utc>,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

#[derive(Deserialize, Debug)]
pub struct Tick {
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "bid")]
    pub bid: f64,
    #[serde(rename = "ask")]
    pub ask: f64,
}

#[derive(Debug)]
pub enum DataRecord {
    Ohlcv(Ohlcv),
    Tick(Tick),
}
