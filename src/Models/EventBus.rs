use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use crate::enums::AppError::AppError;
use crate::enums::EventTransaction::EventTransaction;
use crate::enums::TypeLogTransaction::TypeLogTransaction;
use crate::Models::Aggregator::{AuditService};
use crate::Models::FraudControl::FraudControl;
use crate::Models::Transaction::Transaction;

pub fn registry_event(audit: Rc<RefCell<AuditService>>) -> Result<(), AppError> {

    let mut list_transaction:Vec<Transaction> = Vec::with_capacity(audit.borrow().history.len());
    let mut fraud_control:HashSet<FraudControl> = HashSet::new();

    for i in audit.borrow().history.iter(){
        list_transaction.push(i.clone());
    }

    for i in list_transaction.iter_mut() {
        for j in audit.borrow_mut().listeners.iter() {
            let transaction = j.borrow_mut().on_transaction(i);
            if transaction.1 == EventTransaction::Blocked{
                fraud_control.insert(FraudControl::new(i.id));
            }
            if transaction.0 == TypeLogTransaction::FRAUD && transaction.1 == EventTransaction::Blocked{
                break
            }
        }
    }

    audit.borrow_mut().fraud_history = Rc::new(RefCell::new(fraud_control));
    Ok(())

}