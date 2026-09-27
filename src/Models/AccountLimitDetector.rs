use std::cell::RefCell;
use std::collections::HashMap;
use crate::enums::EventTransaction::EventTransaction;
use crate::enums::TypeLogTransaction::TypeLogTransaction;
use crate::Models::Aggregator::EventListener;
use crate::Models::LimitAndTransactions::LimitAndTransaction;
use crate::Models::Transaction::Transaction;

pub struct AccountLimitDetector {
    pub limit_by_account: RefCell<HashMap<u32, LimitAndTransaction>>
}

impl AccountLimitDetector {

    pub fn new() -> Self{
        Self{
            limit_by_account: RefCell::new(HashMap::new())
        }
    }

    pub fn update_limit_values(&mut self, accountid: u32) -> () {

    }
}

impl EventListener for AccountLimitDetector{
    fn on_transaction(&mut self, tx: &mut Transaction) -> (TypeLogTransaction, EventTransaction) {


        (TypeLogTransaction::ACCOUNT_LIMIT, EventTransaction::Continue)
    }
}