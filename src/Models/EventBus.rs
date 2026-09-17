use std::cell::RefCell;
use std::rc::Rc;
use crate::enums::AppError::AppError;
use crate::Models::Aggregator::{AuditService, EventListener};
use crate::Models::Transaction::Transaction;

pub fn registry_event(id_transaction: u32, audit: Rc<RefCell<AuditService>>) -> Result<(), AppError> {


    let transaction = audit.borrow().history.iter()
        .find(|x| x.id == id_transaction)
        .unwrap_or(&Transaction::default()).clone();
    if transaction.id == 0 {
        return Err(AppError::RuntimeError(String::from("No transaction found in history")));
    }

    for i in &audit.try_borrow_mut()
        .map_err(|e| AppError::RuntimeError(e.to_string()))?.listeners{
        i.borrow_mut().on_transaction(&transaction);
    }

    Ok(())

}