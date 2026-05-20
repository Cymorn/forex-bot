#[derive(Debug, Clone)]
pub struct Candle {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub start_time: u64,
    pub end_time: u64,
}

impl Candle {
    pub fn new(price: f64, timestamp: u64) -> Self {
        Self {
            open: price,
            high: price,
            low: price,
            close: price,
            start_time: timestamp,
            end_time: timestamp,
        }
    }

    pub fn update(&mut self, price: f64, timestamp: u64) {
        if price > self.high {
            self.high = price;
        }
        if price < self.low {
            self.low = price;
        }

        self.close = price;
        self.end_time = timestamp;
    }
}