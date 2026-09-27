use crate::Models::RejectReason::RejectReason;

#[derive(Debug, Clone, Default)]
pub struct Transaction {
    pub id: u32,
    pub amount: f64,
    pub account_id: u32, //identificador do numero da conta
    pub rejection_reasons: Vec<RejectReason> //lista de razoes para rejeicao
}

impl Transaction {
    pub fn new(id: u32, amount: f64, account_id: u32) -> Transaction {
        Self{
            id, amount, account_id, rejection_reasons: vec![]
        }
    }

    pub fn add_rejection(&mut self, reason: RejectReason) {
        self.rejection_reasons.push(reason);
    }

}