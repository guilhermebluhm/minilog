use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::enums::AppError::AppError;
use crate::enums::EventTransaction::EventTransaction;
use crate::enums::TypeLogTransaction::TypeLogTransaction;
use crate::Models::Aggregator::{AuditService, EventListener};
use crate::Models::Transaction::Transaction;

pub fn registry_event(id_transaction: u32, audit: Rc<RefCell<AuditService>>) -> Result<(), AppError> {

    //proximos passos: pensar em tornar persistente entre as transacoes
    let mut fraud_ocurrence:HashMap<usize, bool> = HashMap::new();

    let transaction = audit.borrow().history.iter()
        .find(|x| x.id == id_transaction)
        .unwrap_or(&Transaction::default()).clone();

    let transaction_pos = audit.borrow().history.iter()
        .position(|x| x.id == id_transaction).unwrap_or(0);

    if transaction.id == 0 {
        return Err(AppError::RuntimeError(String::from("No transaction found in history")));
    }

    for i in &audit.try_borrow_mut()
        .map_err(|e| AppError::RuntimeError(e.to_string()))?.listeners{

        let event = i.borrow_mut().on_transaction(&transaction);
        if i.borrow().type_operation() == TypeLogTransaction::FRAUD && event == EventTransaction::Blocked{
            println!("Fraude detectada");
            fraud_ocurrence.insert(transaction_pos, true);
            break;
        }

    }

    //pois ao flagear o objeto permite classificar no relatorio
    //consolidado apenas o que e valido
    for (k,_) in fraud_ocurrence{
        audit.borrow_mut().history.get_mut(k).unwrap().flagged = true;
    }

    Ok(())

}