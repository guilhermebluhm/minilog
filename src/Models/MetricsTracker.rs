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
    fn on_transaction(&mut self, tx: &Transaction) -> EventTransaction {
        self.total_volume += tx.amount;
        self.total_count += 1;
        println!("volume processado: {} - (Total: {})", tx.amount, self.total_count);
        EventTransaction::Continue
    }

    fn type_operation(&self) -> TypeLogTransaction {
        TypeLogTransaction::METRICS
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