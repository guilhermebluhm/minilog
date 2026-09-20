#[derive(Debug, Copy, Clone, Default)]
pub struct Transaction {
    pub id: u32,
    pub amount: f64,
}

impl Transaction {
    pub fn new(id: u32, amount: f64) -> Transaction {
        Self{
            id, amount
        }
    }
}