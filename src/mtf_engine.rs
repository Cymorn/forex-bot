use crate::structure_state::Trend;
use crate::signal::Signal;

pub fn mtf_confirm(
    higher_trend: &Trend,
    lower_signal: Signal,
) -> Signal {

    match (higher_trend, lower_signal) {

        (Trend::Bullish, Signal::BUY) => Signal::BUY,

        (Trend::Bearish, Signal::SELL) => Signal::SELL,

        _ => Signal::HOLD,
    }
}