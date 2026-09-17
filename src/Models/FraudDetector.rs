use crate::Models::Aggregator::EventListener;
use crate::Models::Transaction::Transaction;

#[derive(Debug, Copy, Clone)]
pub struct FraudDetector{
    pub fraud_count: usize,
}

impl EventListener for FraudDetector{
    fn on_transaction(&mut self, tx: &Transaction){
        
        if tx.amount > 5000.00 {
            self.fraud_count += 1;
            println!("ALERTA: Fraude detectada!")
        }
        
        if self.fraud_count.eq(&0){
            println!("Transação normal");
        }
        
    }
}

impl FraudDetector{
    pub fn new() -> Self{
        Self{
            fraud_count: 0
        }
    }
}

