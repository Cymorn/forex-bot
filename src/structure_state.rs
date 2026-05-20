#[derive(Debug, Clone, PartialEq)]
pub enum Trend {
    Bullish,
    Bearish,
    Neutral,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MarketStructure {
    BOS,
    CHOCH,
    NONE,
}

pub struct StructureState {
    pub trend: Trend,

    pub last_high: f64,
    pub last_low: f64,

    pub previous_high: f64,
    pub previous_low: f64,

    pub sweep_count: u8,

    pub structure: MarketStructure,
}

impl StructureState {
    pub fn new() -> Self {
        Self {
            trend: Trend::Neutral,

            last_high: 0.0,
            last_low: f64::MAX,

            previous_high: 0.0,
            previous_low: f64::MAX,

            sweep_count: 0,

            structure: MarketStructure::NONE,
        }
    }
}