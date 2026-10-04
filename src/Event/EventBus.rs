use crate::Models::Aggregator::AuditService;
use crate::Models::Transaction::Transaction;
use crate::enums::AppError::AppError;
use crate::enums::EventTransaction::EventTransaction;
use crate::enums::TypeLogTransaction::TypeLogTransaction;
use std::cell::RefCell;
use std::rc::Rc;
use crate::enums::SeverityLevel::SeverityLevel;

pub fn registry_event(audit: Rc<RefCell<AuditService>>) -> Result<(), AppError> {
    let mut list_transaction:Vec<Transaction> = Vec::with_capacity(audit.borrow().history.len());

    for i in audit.borrow().history.iter(){
        list_transaction.push(i.clone());
    }

    for i in list_transaction.iter_mut() {
        for j in audit.borrow_mut().listeners.iter() {



            let trans = j.borrow_mut().on_transaction(i);
            match trans.1 {
                EventTransaction::Blocked(severity) => {
                    if severity == SeverityLevel::ERROR {
                        println!("transação com severidade alta:\n {:#?}", i);
                        break;
                    }
                }
                _ => {}
            }

        }
    }

    audit.borrow_mut().history = list_transaction;
    Ok(())

}