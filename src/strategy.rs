#[derive(Debug)]
pub enum Signal {
    Buy,
    Sell,
    Hold,
}

pub fn sma(prices: &Vec<f64>) -> f64 {
    let sum: f64 = prices.iter().sum();
    sum / prices.len() as f64
}

pub fn generate_signal(current: f64, sma: f64) -> Signal {
    if current > sma {
        Signal::Buy
    } else if current < sma {
        Signal::Sell
    } else {
        Signal::Hold
    }
}