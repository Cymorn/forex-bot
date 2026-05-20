mod market_deriv;
mod signal;
mod candle;
mod candle_builder;
mod smc;
mod structure_state;
mod entry_state;
mod strategy_engine;
mod execution;
mod risk_manager;
mod state;
mod timeframe;
mod trend_filter;
mod mtf_engine;
mod bias;
mod multi_tf_candles;

use state::AppState;
use candle_builder::CandleBuilder;
use structure_state::StructureState;
use entry_state::EntryState;
use risk_manager::RiskManager;
use execution::ExecutionEngine;
use strategy_engine::generate_signal;
use mtf_engine::mtf_confirm;
use bias::{calculate_bias, Bias};
use multi_tf_candles::MultiTfCandles;
use timeframe::Timeframe;

#[tokio::main]
async fn main() {
    let state = AppState::new();

    tokio::spawn(market_deriv::start_market(state.clone()));

    let mut builder = CandleBuilder::new(60);
    let mut mtf = MultiTfCandles::new();

    let mut structure = StructureState::new();
    let mut entry = EntryState::new();

    
    let mut d1_state = StructureState::new();
    let mut h4_state = StructureState::new();
    let mut h1_state = StructureState::new();

    let risk = RiskManager::new(5, 50.0, 5);
    let mut engine = ExecutionEngine::new(risk);

    println!("🚀 Trading bot started with FULL STRUCTURE ENGINE...");


    loop {
        let price = state.get_price();

        if price == 0.0 {
            continue;
        }

       
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        
        builder.update(price, timestamp);
        let candles = builder.get_candles();

        
        if let Some(candle) = candles.last() {
            mtf.push(Timeframe::M1, candle.clone());
        }

       
        let raw_signal =
            generate_signal(candles, &mut structure, &mut entry);

       
        let higher_trend = &h1_state.trend;
        let signal = mtf_confirm(higher_trend, raw_signal);

       
        let bias = calculate_bias(
            &d1_state.trend,
            &h4_state.trend,
            &h1_state.trend,
        );

       
        match bias {
            Bias::StrongBullish
            | Bias::Bullish
            | Bias::StrongBearish
            | Bias::Bearish => {
                engine.execute(signal, price).await;
            }

            Bias::Neutral => {
                println!("⚠️ Market unclear - no trade");
            }
        }

       
        println!(
            "Price: {}, Signal: {:?}, Bias: {:?}",
            price,
            signal,
            bias
        );

        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}