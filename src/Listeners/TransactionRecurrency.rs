use std::cell::{Ref, RefCell, RefMut};
use crate::enums::EventTransaction::EventTransaction;
use crate::enums::SeverityLevel::SeverityLevel;
use crate::enums::TypeLogTransaction::TypeLogTransaction;
use crate::Models::Aggregator::EventListener;
use crate::Models::RejectReason::RejectReason;
use crate::Models::Transaction::Transaction;

pub struct TransactionRecurrency {
    pub recurrency: RefCell<Vec<(u32, u32)>>
}

impl TransactionRecurrency {
    pub fn new() -> Self {
        Self{
            recurrency: RefCell::new(vec![])
        }
    }

    fn state_recurrency(&self) -> Ref<Vec<(u32, u32)>> {
        self.recurrency.borrow()
    }

    fn insert_new_record(&mut self) -> RefMut<Vec<(u32, u32)>> {
        self.recurrency.borrow_mut()
    }

}

impl EventListener for TransactionRecurrency{
    fn on_transaction(&mut self, tx: &mut Transaction) -> (TypeLogTransaction, EventTransaction) {

        if !self.state_recurrency().is_empty(){

            let mut anterior = 0;
            let ret = self.state_recurrency().iter().fold(0, |mut acc, hash|{

                if hash.0.eq(&tx.account_id){
                    //verificar valores recorrentes
                    if anterior == 0{
                        anterior = hash.1;
                    }
                    else{
                        //pois *hash.1 sempre estará na iteração n+1
                        if anterior == hash.1{
                            acc += 1;       
                        }
                        anterior = hash.1;
                    }
                }
                return acc

            });

            self.insert_new_record().push((tx.account_id, tx.amount as u32));

            match ret { 
                0..=2 => {
                    return (TypeLogTransaction::RECURRENCY_LIMIT, EventTransaction::Continue(SeverityLevel::OK))
                }
                _ => {
                    println!("transações recorrentes suspeitas para a conta: {}. número de iterações: {}", tx.account_id, ret);
                    tx.add_rejection(RejectReason{
                        id: tx.rejection_reasons.len()+1, 
                        reason: format!("transações recorrentes suspeitas para a conta: {}. número de iterações: {}", tx.account_id, ret),
                        type_reject: TypeLogTransaction::RECURRENCY_LIMIT,
                        severity: SeverityLevel::WARNING});
                        return (TypeLogTransaction::RECURRENCY_LIMIT, EventTransaction::Blocked(SeverityLevel::WARNING))
                }
            }

        }
        self.insert_new_record().push((tx.account_id, tx.amount as u32));
        (TypeLogTransaction::RECURRENCY_LIMIT, EventTransaction::Continue(SeverityLevel::OK))
    }
}