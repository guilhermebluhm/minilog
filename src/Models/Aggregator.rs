use std::cell::RefCell;
use std::rc::Rc;
use crate::enums::EventTransaction::EventTransaction;
use crate::enums::TypeLogTransaction::TypeLogTransaction;
use crate::Models::Transaction::Transaction;

pub struct Terminal{
    pub id: u32,
    pub service: Rc<RefCell<AuditService>>
}

#[derive(Clone)]
pub struct AuditService{
    pub history: Vec<Transaction>,
    pub listeners: Vec<Rc<RefCell<dyn EventListener>>>
}

pub trait EventListener{
    fn on_transaction(&mut self, tx: &Transaction) -> EventTransaction;
    fn type_operation(&self) -> TypeLogTransaction;
}

impl Terminal{
    pub fn new(id: u32, service: &Rc<RefCell<AuditService>>) -> Self{
        Self{id, service: service.clone()}
    }

    pub fn process_payment(&self, tx_id: u32, amount: f64, flagged: bool) -> Box<Transaction>{
        let transc = Transaction::new(tx_id, amount, flagged);
        println!("Processando a transação: {:#?}", transc);
        Box::new(transc)
    }

}

impl AuditService{
    pub fn new() -> Self{
        Self{
            history: vec![],
            listeners: vec![],
        }
    }

    pub fn subscribe_listener(&mut self, listener: Rc<RefCell<dyn EventListener>>){
        self.listeners.push(listener);
    }

    pub fn subscribe_transaction(&mut self, tx: Transaction){
        self.history.push(tx);
    }

}