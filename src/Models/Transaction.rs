#[derive(Debug, Copy, Clone, Default)]
pub struct Transaction {
    pub id: u32,
    pub amount: f64,
    pub flagged: bool,
}

impl Transaction {
    pub fn new(id: u32, amount: f64, flagged: bool) -> Transaction {
        Self{
            id, amount, flagged
        }
    }
}