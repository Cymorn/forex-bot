use std::time::{Duration, Instant};

#[derive(Debug, Clone, PartialEq)]
pub enum RiskMode {
    Aggressive,
    Normal,
    Defensive,
}

pub struct RiskManager {
    pub max_trades: u32,
    pub trades_taken: u32,

    pub max_loss: f64,
    pub current_loss: f64,

    pub last_trade_time: Option<Instant>,
    pub cooldown: Duration,

    // AI STATE
    pub mode: RiskMode,
    pub win_streak: u32,
    pub loss_streak: u32,
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

            mode: RiskMode::Normal,
            win_streak: 0,
            loss_streak: 0,
        }
    }

    pub fn can_trade(&self) -> bool {
        if self.current_loss <= -self.max_loss {
            return false;
        }

        if self.trades_taken >= self.max_trades {
            return false;
        }

        if let Some(last) = self.last_trade_time {
            if last.elapsed() < self.effective_cooldown() {
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

        if pnl > 0.0 {
            self.win_streak += 1;
            self.loss_streak = 0;
        } else {
            self.loss_streak += 1;
            self.win_streak = 0;
        }

        self.update_mode();
    }

    
    fn update_mode(&mut self) {
        self.mode = if self.loss_streak >= 3 {
            RiskMode::Defensive
        } else if self.win_streak >= 3 {
            RiskMode::Aggressive
        } else {
            RiskMode::Normal
        };
    }

    pub fn effective_cooldown(&self) -> Duration {
        match self.mode {
            RiskMode::Aggressive => Duration::from_secs(self.cooldown.as_secs().saturating_div(2)),
            RiskMode::Normal => self.cooldown,
            RiskMode::Defensive => Duration::from_secs(self.cooldown.as_secs() * 2),
        }
    }

    pub fn mode(&self) -> RiskMode {
        self.mode.clone()
    }
}