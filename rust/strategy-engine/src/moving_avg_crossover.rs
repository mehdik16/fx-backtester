use data_loader::types::{Position, Tick};
use metrics::indicator::TickIndicator;
use metrics::moving_average::MovingAverage;
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
                    let position = Position::new(
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
    use data_loader::types::Tick;

    // Helper function to create a ticks
    fn create_ticks() -> Vec<Tick> {
        vec![
            Tick {
                bid: 1.0,
                ask: 1.0,
                timestamp: Default::default(),
            },
            Tick {
                bid: 2.0,
                ask: 2.0,
                timestamp: Default::default(),
            },
            Tick {
                bid: 3.0,
                ask: 3.0,
                timestamp: Default::default(),
            },
            Tick {
                bid: 4.0,
                ask: 4.0,
                timestamp: Default::default(),
            },
            Tick {
                bid: 5.0,
                ask: 5.0,
                timestamp: Default::default(),
            },
            Tick {
                bid: 2.0,
                ask: 2.0,
                timestamp: Default::default(),
            },
            Tick {
                bid: 3.0,
                ask: 3.0,
                timestamp: Default::default(),
            },
            Tick {
                bid: 2.0,
                ask: 2.0,
                timestamp: Default::default(),
            },
            Tick {
                bid: 5.0,
                ask: 5.0,
                timestamp: Default::default(),
            },
            Tick {
                bid: 1.0,
                ask: 1.0,
                timestamp: Default::default(),
            },
            Tick {
                bid: 3.0,
                ask: 3.0,
                timestamp: Default::default(),
            },
            Tick {
                bid: 4.0,
                ask: 4.0,
                timestamp: Default::default(),
            },
            Tick {
                bid: 5.0,
                ask: 5.0,
                timestamp: Default::default(),
            },
            Tick {
                bid: 6.0,
                ask: 6.0,
                timestamp: Default::default(),
            },
            Tick {
                bid: 4.0,
                ask: 4.0,
                timestamp: Default::default(),
            },
            Tick {
                bid: 2.0,
                ask: 2.0,
                timestamp: Default::default(),
            },
            Tick {
                bid: 2.0,
                ask: 2.0,
                timestamp: Default::default(),
            },
            Tick {
                bid: 2.0,
                ask: 2.0,
                timestamp: Default::default(),
            },
        ]
    }

    // Test the moving average crossover strategy
    #[test]
    fn test_moving_average_crossover_strategy() {
        let mut strategy = MovingAverageCrossoverStrategy::new(
            3,
            5,
            1000.0,
            Position::new("EUR".to_string(), "USD".to_string(), 1000.0, 0.0),
        );
        let ticks = create_ticks();
        for tick in ticks {
            strategy.on_tick(&tick);
        }
        // Check if the position was updated correctly
        assert!(strategy.position.is_some());
        let position = strategy.position.unwrap();
        // Call position.display to print the position details as part of the test output
        position.display();

        assert!(position.domestic_ccy_amount > 0.0); // Should have bought
    }
}
