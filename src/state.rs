use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AppState {
    pub price: Arc<Mutex<f64>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            price: Arc::new(Mutex::new(0.0)),
        }
    }

    pub fn update_price(&self, value: f64) {
        let mut price = self.price.lock().unwrap();
        *price = value;
    }

    pub fn get_price(&self) -> f64 {
        *self.price.lock().unwrap()
    }
}