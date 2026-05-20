use crate::candle::Candle;

pub struct CandleBuffer {
    pub candles: Vec<Candle>,
    pub max_size: usize,
}

impl CandleBuffer {
    pub fn new(max_size: usize) -> Self {
        Self {
            candles: Vec::new(),
            max_size,
        }
    }

    pub fn add(&mut self, candle: Candle) {
        self.candles.push(candle);

        if self.candles.len() > self.max_size {
            self.candles.remove(0);
        }
    }

    pub fn last(&self) -> Option<&Candle> {
        self.candles.last()
    }
}