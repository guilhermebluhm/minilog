use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;
use crate::enums::AppError::AppError;
use crate::enums::ClientTier::ClientTier;
use crate::Models::AccountLimitDetector::AccountLimitDetector;
use crate::Models::Aggregator::{AuditService, EventListener, Terminal};
use crate::Models::EventBus::registry_event;
use crate::Models::FraudDetector::FraudDetector;
use crate::Models::MetricsTracker::MetricsTracker;
use crate::Models::TransactionRecurrency::TransactionRecurrency;

pub mod Models;
mod enums;

fn main() -> Result<(), AppError> {
    
    let fraud_transaction:      Rc<RefCell<FraudDetector>>         = Rc::new(RefCell::new(FraudDetector::new()));
    let metrics_transaction:    Rc<RefCell<MetricsTracker>>        = Rc::new(RefCell::new(MetricsTracker::new()));
    let account_limit_metrics:  Rc<RefCell<AccountLimitDetector>>  = Rc::new(RefCell::new(AccountLimitDetector::new()));
    let transaction_recurrency: Rc<RefCell<TransactionRecurrency>> = Rc::new(RefCell::new(TransactionRecurrency::new()));

    let aud:            AuditService                       = AuditService::new();
    let ref_aud:        Rc<RefCell<AuditService>>          = Rc::new(RefCell::new(aud));
    let terminal:       Terminal                           = Terminal::new(1, &ref_aud);

    let transaction_1 = terminal.process_payment(1, 600.00,5122, ClientTier::BASIC);
    let transaction_2 = terminal.process_payment(2, 2550.00,5122, ClientTier::BASIC);
    let transaction_3 = terminal.process_payment(3, 50.00,5122, ClientTier::BASIC);
    let transaction_4 = terminal.process_payment(4, 50.00,5122, ClientTier::BASIC);
    let transaction_5 = terminal.process_payment(5, 50.00,5122, ClientTier::BASIC);
    let transaction_6 = terminal.process_payment(6, 50.00,5122, ClientTier::BASIC);
    let transaction_7 = terminal.process_payment(7, 50.00,5122, ClientTier::BASIC);

    terminal.service.try_borrow_mut()
        .map_err(|e| AppError::RuntimeError(e.to_string()))?
        .subscribe_listener(account_limit_metrics.clone());
    terminal.service.try_borrow_mut()
        .map_err(|e| AppError::RuntimeError(e.to_string()))?
        .subscribe_listener(transaction_recurrency.clone());
    terminal.service.try_borrow_mut()
        .map_err(|e| AppError::RuntimeError(e.to_string()))?
        .subscribe_listener(fraud_transaction.clone());
    terminal.service.try_borrow_mut()
        .map_err(|e| AppError::RuntimeError(e.to_string()))?
        .subscribe_listener(metrics_transaction.clone());

    terminal.service.borrow_mut()
        .subscribe_transaction(*transaction_1.clone());
    terminal.service.borrow_mut()
        .subscribe_transaction(*transaction_2.clone());
    terminal.service.borrow_mut()
        .subscribe_transaction(*transaction_3.clone());
    terminal.service.borrow_mut()
        .subscribe_transaction(*transaction_4.clone());
    terminal.service.borrow_mut()
        .subscribe_transaction(*transaction_5.clone());
    terminal.service.borrow_mut()
        .subscribe_transaction(*transaction_6.clone());
    terminal.service.borrow_mut()
        .subscribe_transaction(*transaction_7.clone());

    let _ = registry_event(terminal.service.clone());
    terminal.service.borrow().get_account_statement(5122);
    Ok(())

}
