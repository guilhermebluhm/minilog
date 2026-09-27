use crate::enums::EventTransaction::EventTransaction;
use crate::enums::TypeLogTransaction::TypeLogTransaction;
use crate::Models::Aggregator::EventListener;
use crate::Models::Transaction::Transaction;

#[derive(Debug, Copy, Clone)]
pub struct MetricsTracker {
    pub total_volume: f64,
    pub total_count: usize
}

impl EventListener for MetricsTracker {
    fn on_transaction(&mut self, tx: &mut Transaction) -> (TypeLogTransaction, EventTransaction) {
        self.total_volume += tx.amount;
        self.total_count += 1;
        println!("volume processado: {} - (Total: {})", self.total_volume, self.total_count);
        (TypeLogTransaction::METRICS, EventTransaction::Continue)
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