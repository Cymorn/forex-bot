use crate::signal::Signal;
use crate::structure_state::StructureState;

pub struct Filters {
    pub min_trend_strength: u8,
    pub allow_trading: bool,
}

impl Filters {
    pub fn new() -> Self {
        Self {
            min_trend_strength: 1,
            allow_trading: true,
        }
    }

    pub fn apply(
        &self,
        signal: Signal,
        state: &StructureState,
        price: f64
    ) -> Signal {

        if !self.allow_trading {
            return Signal::HOLD;
        }

        
        match state.trend {
            crate::structure_state::Trend::Neutral => {
                return Signal::HOLD;
            }
            _ => {}
        }

        
        if state.sweep_count == 0 {
            return Signal::HOLD;
        }

        
        if price <= 0.0 {
            return Signal::HOLD;
        }

    
        signal
    }
}