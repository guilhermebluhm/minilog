use crate::Models::Aggregator::AuditService;
use crate::dispatch::DynamicThreshold::avaliable_branch_status;
use crate::enums::AppError::AppError;
use crate::logger::LoggerCore::LoggerLogic;
use std::cell::RefCell;
use std::rc::Rc;
use crate::event::AlertNotification::generate_notification;

pub fn registry_event(audit: Rc<RefCell<AuditService>>) -> Result<(), AppError> {
    let mut list_transaction: Vec<Box<dyn LoggerLogic>> = Vec::with_capacity(audit.borrow().history.len());

    for i in audit.borrow().history.iter(){
        let transaction_in_box = Box::new(i.clone());
        list_transaction.push(transaction_in_box);
    }

    avaliable_branch_status(&mut list_transaction);
    generate_notification(&list_transaction);

    for i in list_transaction.iter_mut() {
        let mut t = audit.borrow_mut().get_mut_instance_transaction(i.transaction_id()).as_mut().clone();

        if i.log_level() >= i.minimal_log_level() {
            for j in audit.borrow_mut().listeners.iter() {
                let _ = j.borrow_mut().on_transaction(&mut t);
            }
            *audit.borrow_mut().history.iter_mut().find(|f| f.id == i.transaction_id()).unwrap() = t;
        }
        else{
            println!("transaction id: {} is not ready for processing logic", i.transaction_id());
        }

    }
    Ok(())

}