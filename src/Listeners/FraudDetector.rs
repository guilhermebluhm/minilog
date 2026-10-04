use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use crate::enums::EventTransaction::EventTransaction;
use crate::enums::SeverityLevel::SeverityLevel;
use crate::enums::TypeLogTransaction::TypeLogTransaction;
use crate::Models::Aggregator::EventListener;
use crate::Models::RejectReason::RejectReason;
use crate::Models::Transaction::Transaction;

#[derive(Debug, Clone)]
pub struct FraudDetector{
    pub fraud_count: usize,
    pub internal_transaction: Rc<RefCell<HashMap<u32, u32>>>
    //precisa vincular a conta para nao produzir falso-positivo
    //por acumulo de transacoes. entao requer ser hashmap (internal_transaction)
}

impl EventListener for FraudDetector{
    fn on_transaction(&mut self, tx: &mut Transaction) -> (TypeLogTransaction, EventTransaction) {

        if list_accountid_black_list().iter().copied().find(|c| c.eq(&tx.account_id)).is_some(){
            self.fraud_count+=1;
            self.internal_transaction.borrow_mut().insert(tx.account_id, tx.amount as u32);
            tx.add_rejection(RejectReason{id: tx.rejection_reasons.len()+1,
                reason: format!("Transação bloqueada ligada a conta suspeita - {}", tx.amount,),
                type_reject: TypeLogTransaction::FRAUD,
                severity: SeverityLevel::WARNING});
            return (TypeLogTransaction::FRAUD, EventTransaction::Blocked(SeverityLevel::WARNING))
        }
        else if self.internal_transaction.borrow().is_empty() && tx.amount > 5000.00{
            self.fraud_count+=1;
            self.internal_transaction.borrow_mut().insert(tx.account_id, tx.amount as u32);
            tx.add_rejection(RejectReason{id: tx.rejection_reasons.len()+1,
                reason: format!("Transação bloqueada por valor suspeito - {}", tx.amount),
                type_reject: TypeLogTransaction::FRAUD,
                severity: SeverityLevel::WARNING});
            return (TypeLogTransaction::FRAUD, EventTransaction::Blocked(SeverityLevel::WARNING))
        }
        else if !self.internal_transaction.borrow().is_empty() && self.internal_transaction.borrow().get(&tx.account_id).is_some(){
            let mut sum = 0.00;
            for i in self.internal_transaction.borrow().iter() {
                sum += *i.1 as f64;
            }
            println!("valor acumulado transações: {:?}", sum);

            if sum != 0.00{
                if tx.amount >= (sum * 2.00) {
                    self.fraud_count+=1;
                    self.internal_transaction.borrow_mut().insert(tx.account_id, tx.amount as u32);
                    tx.add_rejection(RejectReason{id: tx.rejection_reasons.len()+1,
                        reason: format!("Transação bloqueada por valor suspeito fora do padrão - {}", sum + tx.amount),
                        type_reject: TypeLogTransaction::FRAUD,
                        severity: SeverityLevel::WARNING});
                    return (TypeLogTransaction::FRAUD, EventTransaction::Blocked(SeverityLevel::WARNING))
                }
            }
        }
        self.internal_transaction.borrow_mut().insert(tx.account_id, tx.amount as u32);
        println!("Transação normal");
        (TypeLogTransaction::FRAUD, EventTransaction::Continue(SeverityLevel::OK))
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
            internal_transaction: Rc::new(RefCell::new(HashMap::new())),
        }
    }
}

