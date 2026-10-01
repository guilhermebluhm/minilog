use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use crate::enums::ClientTier::ClientTier;
use crate::enums::EventTransaction::EventTransaction;
use crate::enums::TypeLogTransaction::TypeLogTransaction;
use crate::Models::Aggregator::EventListener;
use crate::Models::LimitAndTransactions::LimitAndTransaction;
use crate::Models::RejectReason::RejectReason;
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

    pub fn get_limit_mutable(&mut self) -> RefMut<HashMap<u32, LimitAndTransaction>> {
        self.limit_by_account.borrow_mut()
    }

    pub fn get_limit(&self) -> Ref<HashMap<u32, LimitAndTransaction>> {
        self.limit_by_account.borrow()
    }

    pub fn update_limit_values(&mut self, accountid: u32) -> f64 {
        let mut sum = 0.00;
        let transaction_for_account = self.get_limit().get(&accountid).map(|c| c.transactions.clone()).unwrap();
        for i in transaction_for_account {
            sum += i.amount;
        }
        sum
    }
}

//por enquanto suporta apenas clientes de nivel básico
impl EventListener for AccountLimitDetector{
    fn on_transaction(&mut self, tx: &mut Transaction) -> (TypeLogTransaction, EventTransaction) {

        if self.get_limit().contains_key(&tx.account_id){

            match tx.client_tier {
                ClientTier::BASIC => {
                    if (self.update_limit_values(tx.account_id) + tx.amount) > 3000.00 {
                        tx.rejection_reasons.push(RejectReason{id: tx.rejection_reasons.len()+1,
                            reason: "Limite estourado".to_string(),
                            type_reject: TypeLogTransaction::ACCOUNT_LIMIT});
                        println!("{:#?}", tx); //detalhar a transação que levou ao bloqueio do fluxo (provisorio desta forma)
                        return (TypeLogTransaction::ACCOUNT_LIMIT, EventTransaction::Blocked)
                    }
                }
                _ => {}
            }

        }
        else{
            let history_transaction = LimitAndTransaction {
                transactions: vec![tx.clone()]
            };
            self.get_limit_mutable().insert(tx.account_id, history_transaction);
        }

        (TypeLogTransaction::ACCOUNT_LIMIT, EventTransaction::Continue)
    }
}