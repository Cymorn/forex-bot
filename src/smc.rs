use crate::candle::Candle;
use crate::signal::Signal;
use crate::entry_state::EntryState;

use crate::structure_state::{
    StructureState,
    Trend,
    MarketStructure,
};

pub fn detect_smc(
    candles: &Vec<Candle>,
    state: &mut StructureState,
    entry: &mut EntryState,
) -> Signal {

    if candles.len() < 15 {
        return Signal::HOLD;
    }

    let last = candles.last().unwrap();

    let recent = &candles[candles.len() - 15..];

    let highest = recent
        .iter()
        .map(|c| c.high)
        .fold(f64::MIN, f64::max);

    let lowest = recent
        .iter()
        .map(|c| c.low)
        .fold(f64::MAX, f64::min);

   
    if last.close > state.previous_high {

        state.structure = MarketStructure::BOS;
        state.trend = Trend::Bullish;

        state.previous_high = last.high;

        println!("📈 Bullish BOS detected");
    }

    
    if last.close < state.previous_low {

        state.structure = MarketStructure::BOS;
        state.trend = Trend::Bearish;

        state.previous_low = last.low;

        println!("📉 Bearish BOS detected");
    }

   
    if state.trend == Trend::Bullish
        && last.close < state.last_low
    {
        state.structure = MarketStructure::CHOCH;
        state.trend = Trend::Bearish;

        println!("🔄 Bearish CHOCH detected");
    }

    if state.trend == Trend::Bearish
        && last.close > state.last_high
    {
        state.structure = MarketStructure::CHOCH;
        state.trend = Trend::Bullish;

        println!("🔄 Bullish CHOCH detected");
    }

    
    if last.high > highest && last.close < highest {
        state.sweep_count += 1;

        println!("🧹 Liquidity sweep HIGH");
    }

    if last.low < lowest && last.close > lowest {
        state.sweep_count += 1;

        println!("🧹 Liquidity sweep LOW");
    }

    
    if entry.active {

        entry.candles_waited += 1;

        println!(
            "⏳ Waiting candles: {} / 5",
            entry.candles_waited
        );

        if entry.candles_waited >= 5 {

            let signal = entry.pending_signal.clone().unwrap();

            entry.reset();

            return signal;
        }

        return Signal::HOLD;
    }

    
    if state.trend == Trend::Bullish
        && state.structure == MarketStructure::BOS
        && state.sweep_count >= 1
    {

        state.sweep_count = 0;

        entry.start(Signal::BUY);

        println!("🟢 BUY setup confirmed");

        return Signal::HOLD;
    }

   
   
    if state.trend == Trend::Bearish
        && state.structure == MarketStructure::BOS
        && state.sweep_count >= 1
    {

        state.sweep_count = 0;

        entry.start(Signal::SELL);

        println!("🔴 SELL setup confirmed");

        return Signal::HOLD;
    }

   
    state.last_high = last.high;
    state.last_low = last.low;

    Signal::HOLD
}