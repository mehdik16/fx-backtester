use crate::indicator::TickIndicator;
use data_loader::types::Tick;

pub struct MovingAverage {
    period: usize,
    values: Vec<f64>,
    sum: f64,
}

impl MovingAverage {
    pub fn new(period: usize) -> Self {
        Self {
            period,
            values: Vec::new(),
            sum: 0.0,
        }
    }
}

impl TickIndicator for MovingAverage {
    fn add_tick(&mut self, tick: &Tick) {
        let mid = (tick.bid + tick.ask) / 2.0;
        self.values.push(mid);
        if self.values.len() > self.period {
            let removed = self.values.remove(0);
            self.sum -= removed;
        }
        self.sum += mid;
    }

    fn value(&self) -> Option<f64> {
        if self.values.len() == self.period {
            Some(self.sum / self.period as f64)
        } else {
            None
        }
    }
}

// Unit tests for MovingAverage
#[cfg(test)]
mod tests {
    use super::*;
    use data_loader::types::Tick;

    #[test]
    fn test_moving_average() {
        let mut ma = MovingAverage::new(3);
        let ticks = vec![
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
        ];

        ma.add_tick(&ticks[0]);
        assert_eq!(ma.value(), None);
        ma.add_tick(&ticks[1]);
        assert_eq!(ma.value(), None);
        ma.add_tick(&ticks[2]);
        assert_eq!(ma.value(), Some(2.0)); // (1 + 2 + 3) / 3
        ma.add_tick(&ticks[3]);
        assert_eq!(ma.value(), Some(3.0)); // (2 + 3 + 4) / 3  
    }
}
