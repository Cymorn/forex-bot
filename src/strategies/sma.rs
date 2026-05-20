use crate::signal::Signal;

pub fn sma(prices: &Vec<f64>) -> f64 {
    let sum: f64 = prices.iter().sum();
    sum / prices.len() as f64
}

pub fn sma_signal(prices: &Vec<f64>) -> Signal {
    if prices.len() < 5 {
        return Signal::HOLD;
    }

    let avg = sma(prices);
    let last = *prices.last().unwrap();

    if last > avg {
        Signal::BUY
    } else if last < avg {
        Signal::SELL
    } else {
        Signal::HOLD
    }
}