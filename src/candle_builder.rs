use crate::candle::Candle;

pub struct CandleBuilder {
    pub candles: Vec<Candle>,
    pub current: Option<Candle>,
    pub interval_secs: u64,
    pub last_timestamp: u64,
}

impl CandleBuilder {
    pub fn new(interval_secs: u64) -> Self {
        Self {
            candles: Vec::new(),
            current: None,
            interval_secs,
            last_timestamp: 0,
        }
    }

    pub fn update(&mut self, price: f64, timestamp: u64) {
        if self.current.is_none() {
            self.current = Some(Candle::new(price, timestamp));
            self.last_timestamp = timestamp;
            return;
        }

        let candle = self.current.as_mut().unwrap();

        // new candle time
        if timestamp - self.last_timestamp >= self.interval_secs {
            self.candles.push(candle.clone());
            self.current = Some(Candle::new(price, timestamp));
            self.last_timestamp = timestamp;
        } else {
            candle.update(price, timestamp);
        }
    }

    pub fn get_candles(&self) -> &Vec<Candle> {
        &self.candles
    }
}