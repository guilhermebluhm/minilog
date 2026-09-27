use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;
use crate::enums::AppError::AppError;
use crate::Models::AccountLimitDetector::AccountLimitDetector;
use crate::Models::Aggregator::{AuditService, EventListener, Terminal};
use crate::Models::EventBus::registry_event;
use crate::Models::FraudDetector::FraudDetector;
use crate::Models::MetricsTracker::MetricsTracker;

pub mod Models;
mod enums;

fn main() -> Result<(), AppError> {

    let aud = AuditService::new();
    let ref_aud:                Rc<RefCell<AuditService>> =         Rc::new(RefCell::new(aud));
    let fraud_transaction:      Rc<RefCell<FraudDetector>> =        Rc::new(RefCell::new(FraudDetector::new()));
    let metrics_transaction:    Rc<RefCell<MetricsTracker>> =       Rc::new(RefCell::new(MetricsTracker::new()));
    let account_limit_metrics:  Rc<RefCell<AccountLimitDetector>> = Rc::new(RefCell::new(AccountLimitDetector::new()));
    let terminal = Terminal::new(1, &ref_aud);

    let transaction_1 = terminal.process_payment(1, 2000.00,5122);
    let transaction_2 = terminal.process_payment(2, 90.00,662);

    terminal.service.try_borrow_mut()
        .map_err(|e| AppError::RuntimeError(e.to_string()))?
        .subscribe_listener(fraud_transaction.clone());
    terminal.service.try_borrow_mut()
        .map_err(|e| AppError::RuntimeError(e.to_string()))?
        .subscribe_listener(metrics_transaction.clone());
    terminal.service.try_borrow_mut()
        .map_err(|e| AppError::RuntimeError(e.to_string()))?
        .subscribe_listener(account_limit_metrics.clone());

    terminal.service.borrow_mut()
        .subscribe_transaction(*transaction_1.clone());
    terminal.service.borrow_mut()
        .subscribe_transaction(*transaction_2.clone());

    let _ = registry_event(terminal.service.clone());
    println!("{:?}", terminal.service.borrow().fraud_history.clone());
    Ok(())

}
