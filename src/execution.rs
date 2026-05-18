use crate::signal::Signal;
use crate::risk_manager::RiskManager;

pub struct ExecutionEngine {
    pub risk: RiskManager,
    pub position: Option<Signal>,
}

impl ExecutionEngine {
    pub fn new(risk: RiskManager) -> Self {
        Self {
            risk,
            position: None,
        }
    }

    pub async fn execute(&mut self, signal: Signal, price: f64) {
        // 🛑 Risk check first
        if !self.risk.can_trade() {
            println!("🚫 Trade blocked by risk manager");
            return;
        }

        match signal {
            Signal::BUY => {
                println!("📈 BUY executed at {}", price);
                self.position = Some(Signal::BUY);
                self.risk.register_trade();
            }

            Signal::SELL => {
                println!("📉 SELL executed at {}", price);
                self.position = Some(Signal::SELL);
                self.risk.register_trade();
            }

            Signal::HOLD => {
                println!("⏸ HOLD - no action");
            }
        }
    }
}