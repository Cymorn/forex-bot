use std::time::{Duration, Instant};

pub struct RiskManager {
    pub max_trades: u32,
    pub trades_taken: u32,

    pub max_loss: f64,
    pub current_loss: f64,

    pub last_trade_time: Option<Instant>,
    pub cooldown: Duration,
}

impl RiskManager {
    pub fn new(max_trades: u32, max_loss: f64, cooldown_secs: u64) -> Self {
        Self {
            max_trades,
            trades_taken: 0,
            max_loss,
            current_loss: 0.0,
            last_trade_time: None,
            cooldown: Duration::from_secs(cooldown_secs),
        }
    }

    pub fn can_trade(&self) -> bool {
        if self.trades_taken >= self.max_trades {
            return false;
        }

        if self.current_loss <= -self.max_loss {
            return false;
        }

        if let Some(last) = self.last_trade_time {
            if last.elapsed() < self.cooldown {
                return false;
            }
        }

        true
    }

    pub fn register_trade(&mut self) {
        self.trades_taken += 1;
        self.last_trade_time = Some(Instant::now());
    }

    pub fn update_pnl(&mut self, pnl: f64) {
        self.current_loss += pnl;
    }
}