mod market;
mod strategy;
mod market_deriv;

use strategy::{generate_signal, sma, Signal};
use std::{thread::sleep, time::Duration};

#[tokio::main]
async fn main() {
    market_deriv::connect_deriv().await;
    
    let mut prices: Vec<f64> = Vec::new();

    loop {
        let data = market::fetch_rates()
            .await
            .expect("Failed to fetch market data");

        let current = data.rates["EUR"];

        prices.push(current);

        if prices.len() > 5 {
            prices.remove(0);
        }

        let signal = if prices.len() < 5 {
            Signal::Hold
        } else {
            let average = sma(&prices);
            generate_signal(current, average)
        };

        println!("Price: {}", current);
        println!("Signal: {:?}", signal);
        println!("----------------------");

        sleep(Duration::from_secs(1));
    }
}