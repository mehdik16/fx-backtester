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

#[derive(Debug)]
pub struct Position {
    domestic_ccy: String,
    foreign_ccy: String,
    domestic_ccy_amount: f64,
    foreign_ccy_amount: f64,
}

impl Position {
    pub fn new(
        domestic_ccy: String,
        foreign_ccy: String,
        domestic_ccy_amount: f64,
        foreign_ccy_amount: f64,
    ) -> Self {
        Self {
            domestic_ccy,
            foreign_ccy,
            domestic_ccy_amount,
            foreign_ccy_amount,
        }
    }

    pub fn buy(&mut self, domestic_amount: f64, fx_rate: f64) {
        let foreign_amount = domestic_amount / fx_rate;
        if self.domestic_ccy_amount >= domestic_amount {
            self.domestic_ccy_amount -= domestic_amount;
            self.foreign_ccy_amount += foreign_amount;
        }
    }

    pub fn sell(&mut self, foreign_amount: f64, fx_rate: f64) {
        let domestic_amount = foreign_amount * fx_rate;
        if self.foreign_ccy_amount >= foreign_amount {
            self.foreign_ccy_amount -= foreign_amount;
            self.domestic_ccy_amount += domestic_amount;
        }
    }

    // TODO: Add function to display current position
    pub fn display(&self) {
        println!(
            "Position: {} {} ({}), {} {} ({})",
            self.domestic_ccy,
            self.domestic_ccy_amount,
            self.domestic_ccy_amount,
            self.foreign_ccy,
            self.foreign_ccy_amount,
            self.foreign_ccy_amount
        );
    }
}
