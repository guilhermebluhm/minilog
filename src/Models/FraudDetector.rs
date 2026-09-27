use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use crate::enums::EventTransaction::EventTransaction;
use crate::enums::TypeLogTransaction::TypeLogTransaction;
use crate::Models::Aggregator::EventListener;
use crate::Models::RejectReason::RejectReason;
use crate::Models::Transaction::Transaction;

#[derive(Debug, Clone)]
pub struct FraudDetector{
    pub fraud_count: usize,
    pub internal_transaction: Rc<RefCell<HashSet<u32>>>
}

impl EventListener for FraudDetector{
    fn on_transaction(&mut self, tx: &mut Transaction) -> (TypeLogTransaction, EventTransaction) {

        if list_accountid_black_list().iter().copied().find(|c| c.eq(&tx.account_id)).is_some(){
            self.fraud_count+=1;
            tx.add_rejection(RejectReason{id: tx.rejection_reasons.len(), reason: format!("Transação bloqueada ligada a conta suspeita - {}", tx.amount)});
            return (TypeLogTransaction::FRAUD, EventTransaction::Blocked)
        }

        if self.internal_transaction.borrow().is_empty() && tx.amount > 5000.00{
            self.fraud_count+=1;
            tx.add_rejection(RejectReason{id: tx.rejection_reasons.len(), reason: format!("Transação bloqueada por valor suspeito - {}", tx.amount)});
            return (TypeLogTransaction::FRAUD, EventTransaction::Blocked)
        }
        else{
            let mut sum = 0.00;
            for i in self.internal_transaction.borrow().iter() {
                sum += *i as f64;
            }
            println!("valor acumulado transações: {:?}", sum);

            if sum != 0.00{
                if tx.amount >= (sum * 2.00) {
                    self.fraud_count+=1;
                    tx.add_rejection(RejectReason{id: tx.rejection_reasons.len(), reason: format!("Transação bloqueada por valor suspeito fora do padrão - {}", tx.amount)});
                    return (TypeLogTransaction::FRAUD, EventTransaction::Blocked)
                }
            }
        }
        self.internal_transaction.borrow_mut().insert(tx.amount as u32);
        println!("Transação normal");
        (TypeLogTransaction::FRAUD, EventTransaction::Continue)
    }
}

fn list_accountid_black_list() -> Vec<u32> {

    vec![
        11223
    ]

}

impl FraudDetector{
    pub fn new() -> Self{
        Self{
            fraud_count: 0,
            internal_transaction: Rc::new(RefCell::new(HashSet::new())),
        }
    }
}

