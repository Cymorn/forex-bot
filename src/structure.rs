use crate::candle::Candle;

#[derive(Debug, Clone, PartialEq)]
pub enum MarketStructure {
    HigherHigh,
    HigherLow,
    LowerHigh,
    LowerLow,
    None,
}

#[derive(Debug)]
pub struct StructurePoint {
    pub price: f64,
    pub index: usize,
    pub kind: MarketStructure,
}

pub struct StructureEngine {
    pub points: Vec<StructurePoint>,
}

impl StructureEngine {
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
        }
    }

    pub fn analyze(&mut self, candles: &Vec<Candle>) {
        if candles.len() < 3 {
            return;
        }

        let len = candles.len();

        let prev = &candles[len - 2];
        let curr = &candles[len - 1];
        let prev2 = &candles[len - 3];

       
        if curr.high > prev.high && prev.high > prev2.high {
            self.points.push(StructurePoint {
                price: curr.high,
                index: len - 1,
                kind: MarketStructure::HigherHigh,
            });
        }

        
        if curr.low < prev.low && prev.low < prev2.low {
            let kind = if curr.low > prev2.low {
                MarketStructure::HigherLow
            } else {
                MarketStructure::LowerLow
            };

            self.points.push(StructurePoint {
                price: curr.low,
                index: len - 1,
                kind,
            });
        }
    }

    pub fn last_points(&self, n: usize) -> Vec<&StructurePoint> {
        self.points.iter().rev().take(n).collect()
    }
}