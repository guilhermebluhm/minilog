use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use crate::enums::ClientTier::ClientTier;
use crate::enums::EventTransaction::EventTransaction;
use crate::enums::TypeLogTransaction::TypeLogTransaction;
use crate::Models::FraudControl::FraudControl;
use crate::Models::Transaction::Transaction;

pub struct Terminal{
    pub id: u32,
    pub service: Rc<RefCell<AuditService>>
}

#[derive(Clone)]
pub struct AuditService{
    pub history: Vec<Transaction>,
    pub listeners: Vec<Rc<RefCell<dyn EventListener>>>,
    pub fraud_history: Rc<RefCell<HashSet<FraudControl>>>
}

pub trait EventListener{
    fn on_transaction(&mut self, tx: &mut Transaction) -> (TypeLogTransaction, EventTransaction);
}

impl Terminal{
    pub fn new(id: u32, service: &Rc<RefCell<AuditService>>) -> Self{
        Self{id, service: service.clone()}
    }

    pub fn process_payment(&self, tx_id: u32, amount: f64, account_id: u32, tier: ClientTier) -> Box<Transaction>{
        let transc = Transaction::new(tx_id, amount, account_id, tier);
        println!("Processando a transação: {:#?}", transc);
        Box::new(transc)
    }
}

impl AuditService{
    pub fn new() -> Self{
        Self{
            history: vec![],
            listeners: vec![],
            fraud_history: Rc::new(RefCell::new(HashSet::new()))
        }
    }

    pub fn subscribe_listener(&mut self, listener: Rc<RefCell<dyn EventListener>>){
        self.listeners.push(listener);
    }

    pub fn subscribe_transaction(&mut self, tx: Transaction){
        self.history.push(tx);
    }

}