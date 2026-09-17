use crate::Models::Aggregator::EventListener;
use crate::Models::Transaction::Transaction;

#[derive(Debug, Copy, Clone)]
pub struct MetricsTracker {
    pub total_volume: f64,
    pub total_count: usize
}

impl EventListener for MetricsTracker {
    fn on_transaction(&mut self, tx: &Transaction) {
        self.total_volume += tx.amount;
        self.total_count += 1;
        
        println!("volume processado: {} - (Total: {})", tx.amount, self.total_count);
    }
}

impl MetricsTracker {
    pub fn new() -> MetricsTracker {
        Self{
            total_count: 0,
            total_volume: 0.00,
        }
    }
}