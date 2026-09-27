use crate::Models::Transaction::Transaction;

#[derive(Debug)]
pub struct LimitAndTransaction{
    pub transactions: Vec<Transaction>,
}

impl LimitAndTransaction{
    pub fn new() -> Self{
        Self{
            transactions: vec![],
        }
    }
}
