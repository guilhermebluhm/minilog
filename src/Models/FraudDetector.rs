use crate::enums::EventTransaction::EventTransaction;
use crate::enums::TypeLogTransaction::TypeLogTransaction;
use crate::Models::Aggregator::EventListener;
use crate::Models::Transaction::Transaction;

#[derive(Debug, Copy, Clone)]
pub struct FraudDetector{
    pub fraud_count: usize,
}

impl EventListener for FraudDetector{
    fn on_transaction(&mut self, tx: &Transaction) -> EventTransaction {
        
        if tx.amount > 5000.00 {
            self.fraud_count += 1;
            return EventTransaction::Blocked
        }
        
        if self.fraud_count.eq(&0){
            println!("Transação normal");
        }
        EventTransaction::Continue
    }

    fn type_operation(&self) -> TypeLogTransaction {
        TypeLogTransaction::FRAUD
    }
}

impl FraudDetector{
    pub fn new() -> Self{
        Self{
            fraud_count: 0
        }
    }
}

