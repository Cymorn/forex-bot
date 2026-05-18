mod market_deriv;
mod strategy;
mod price_buffer;
mod state;
mod execution;
mod signal;
mod risk_manager;

use state::AppState;
use price_buffer::PriceBuffer;
use strategy::generate_signal;
use risk_manager::RiskManager;
use execution::ExecutionEngine;

#[tokio::main]
async fn main() {
    let state = AppState::new();

    tokio::spawn(market_deriv::start_market(state.clone()));

    let mut buffer = PriceBuffer::new(20);

    let risk = RiskManager::new(5, 50.0, 5);
    let mut engine = ExecutionEngine::new(risk);

    println!("🚀 Trading bot started...");

    loop {
        let price = state.get_price();

        if price == 0.0 {
            continue;
        }

        buffer.add(price);

        let signal = generate_signal(&buffer.prices);

        engine.execute(signal, price).await;

        println!("Price: {}, Signal: {:?}", price, signal);

        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}