use crate::candle::Candle;
use crate::signal::Signal;
use crate::smc::detect_smc;
use crate::structure_state::StructureState;
use crate::entry_state::EntryState;

pub fn generate_signal(
    candles: &Vec<Candle>,
    state: &mut StructureState,
    entry: &mut EntryState
) -> Signal {
    detect_smc(candles, state, entry)
}