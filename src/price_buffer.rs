pub struct PriceBuffer {
    pub prices: Vec<f64>,
    capacity: usize,
}

impl PriceBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            prices: Vec::with_capacity(capacity),
            capacity,
        }
    }

    pub fn add(&mut self, price: f64) {
        if self.prices.len() >= self.capacity {
            self.prices.remove(0);
        }
        self.prices.push(price);
    }

    pub fn len(&self) -> usize {
        self.prices.len()
    }

    pub fn is_ready(&self) -> bool {
        self.prices.len() >= self.capacity
    }
}