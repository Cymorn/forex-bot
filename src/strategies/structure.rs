use crate::candle::Candle;
use crate::signal::Signal;

pub fn detect_structure(candles: &Vec<Candle>) -> Signal {
    if candles.len() < 10 {
        return Signal::HOLD;
    }

    let recent = &candles[candles.len() - 10..];

    let highest_high = recent.iter().map(|c| c.high).fold(f64::MIN, f64::max);
    let lowest_low = recent.iter().map(|c| c.low).fold(f64::MAX, f64::min);

    let last = candles.last().unwrap();

    if last.close > highest_high {
        return Signal::BUY;
    }

    if last.close < lowest_low {
        return Signal::SELL;
    }

    Signal::HOLD
}