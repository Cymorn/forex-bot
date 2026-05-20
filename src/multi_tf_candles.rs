use std::collections::HashMap;
use crate::candle::Candle;
use crate::timeframe::Timeframe;

pub struct MultiTfCandles {
    pub data: HashMap<Timeframe, Vec<Candle>>,
}

impl MultiTfCandles {
    pub fn new() -> Self {
        let mut data = HashMap::new();

        data.insert(Timeframe::M1, vec![]);
        data.insert(Timeframe::M5, vec![]);
        data.insert(Timeframe::M15, vec![]);
        data.insert(Timeframe::H1, vec![]);
        data.insert(Timeframe::D1, vec![]);

        Self { data }
    }

    pub fn push(&mut self, tf: Timeframe, candle: Candle) {
        if let Some(list) = self.data.get_mut(&tf) {
            list.push(candle);

            
            if list.len() > 200 {
                list.remove(0);
            }
        }
    }

    pub fn get(&self, tf: Timeframe) -> Vec<Candle> {
        self.data.get(&tf).cloned().unwrap_or_default()
    }
}