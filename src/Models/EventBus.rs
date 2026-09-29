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

    for i in audit.borrow().history.iter(){
        list_transaction.push(i.clone());
    }

    for i in list_transaction.iter_mut() {
        for j in audit.borrow_mut().listeners.iter() {

            let transaction = j.borrow_mut().on_transaction(i);

            //account limit
            if transaction.0 == TypeLogTransaction::ACCOUNT_LIMIT && transaction.1 == EventTransaction::Blocked{
                continue;
            }
            //transaction recurrency
            if transaction.0 == TypeLogTransaction::RECURRENCY_LIMIT && transaction.1 == EventTransaction::Blocked{
                continue;
            }
            //fraud detector
            if transaction.0 == TypeLogTransaction::FRAUD && transaction.1 == EventTransaction::Blocked{
                continue;
            }
        }
    }

    Ok(())

}