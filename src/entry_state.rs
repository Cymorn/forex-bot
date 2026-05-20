use crate::signal::Signal;

pub struct EntryState {
    pub active: bool,
    pub pending_signal: Option<Signal>,
    pub candles_waited: u8,
}

impl EntryState {
    pub fn new() -> Self {
        Self {
            active: false,
            pending_signal: None,
            candles_waited: 0,
        }
    }

    pub fn start(&mut self, signal: Signal) {
        self.active = true;
        self.pending_signal = Some(signal);
        self.candles_waited = 0;
    }

    pub fn reset(&mut self) {
        self.active = false;
        self.pending_signal = None;
        self.candles_waited = 0;
    }
}