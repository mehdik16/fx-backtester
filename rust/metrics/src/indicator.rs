use data_loader::types::{Ohlcv, Tick};

pub trait TickIndicator {
    fn add_tick(&mut self, tick: &Tick);
    fn value(&self) -> Option<f64>;
}

pub trait OhlcvIndicator {
    fn add_ohlcv(&mut self, ohlcv: &Ohlcv);
    fn value(&self) -> Option<f64>;
}
