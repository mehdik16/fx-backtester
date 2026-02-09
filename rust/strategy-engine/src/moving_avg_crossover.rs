pub struct MovingAverageCrossoverStrategy {
    short_ma: MovingAverage,
    long_ma: MovingAverage,
    position: Option<Position>,
    initial_capital: f64,
}

impl MovingAverageCrossoverStrategy {
    pub fn new(
        short_period: usize,
        long_period: usize,
        initial_capital: f64,
        position: Position,
    ) -> Self {
        Self {
            short_ma: MovingAverage::new(short_period),
            long_ma: MovingAverage::new(long_period),
            position: Some(position),
            initial_capital,
        }
    }

    pub fn on_tick(&mut self, tick: &Tick) {
        self.short_ma.add_tick(tick);
        self.long_ma.add_tick(tick);

        if let (Some(short_ma_value), Some(long_ma_value)) =
            (self.short_ma.value(), self.long_ma.value())
        {
            if short_ma_value > long_ma_value {
                // Golden cross - buy signal
                if self.position.is_none() {
                    let mut position = Position::new(
                        "EUR".to_string(),
                        "USD".to_string(),
                        self.initial_capital,
                        0.0,
                    );
                    self.position = Some(position);
                }
                // initiate buy logic here
                let amount_to_buy = self.initial_capital * 0.1; // Example: buy with 10% of capital
                self.position.as_mut().unwrap().buy(amount_to_buy, tick.ask);
            } else if short_ma_value < long_ma_value {
                // Death cross - sell signal
                if let Some(position) = &mut self.position {
                    // initiate sell logic here
                    let amount_to_sell = position.foreign_ccy_amount;
                    position.sell(amount_to_sell, tick.bid);
                }
            }
        }
    }
}

// Unit tests for MovingAverageCrossoverStrategy
#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Ohlcv, Tick};
}
