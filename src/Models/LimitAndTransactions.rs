use crate::Models::Transaction::Transaction;

pub struct LimitAndTransaction{
    pub transactions: Vec<Transaction>,
    pub global_limit_for_client: f64
}

