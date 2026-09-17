use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;
use crate::enums::AppError::AppError;
use crate::Models::Aggregator::{AuditService, EventListener, Terminal};
use crate::Models::EventBus::registry_event;
use crate::Models::FraudDetector::FraudDetector;
use crate::Models::MetricsTracker::MetricsTracker;

pub mod Models;
mod enums;

fn main() -> Result<(), AppError> {

    let aud = AuditService::new();
    let ref_aud:                Rc<RefCell<AuditService>> = Rc::new(RefCell::new(aud));
    let fraud_transaction:      Rc<RefCell<FraudDetector>> = Rc::new(RefCell::new(FraudDetector::new()));;
    let metrics_transaction:    Rc<RefCell<MetricsTracker>> = Rc::new(RefCell::new(MetricsTracker::new()));
    let terminal = Terminal::new(1, &ref_aud);

    let transaction_1 = terminal.process_payment(1, 2000.00, false);
    let transaction_2 = terminal.process_payment(2, 9000.00, false);

    terminal.service.try_borrow_mut()
        .map_err(|e| AppError::RuntimeError(e.to_string()))?
        .subscribe_listener(fraud_transaction.clone());
    terminal.service.try_borrow_mut()
        .map_err(|e| AppError::RuntimeError(e.to_string()))?
        .subscribe_listener(metrics_transaction.clone());

    terminal.service.borrow_mut().subscribe_transaction(*transaction_1.clone());
    registry_event(1, ref_aud.clone())?;
    terminal.service.borrow_mut().subscribe_transaction(*transaction_2.clone());
    registry_event(2, ref_aud.clone())?;

    Ok(())

}
