#[derive(Debug, Clone)]
pub struct RejectReason {
    pub id: usize,
    pub reason: String
}

trait Helpers{
    fn id(&self) -> usize;
    
}

impl Helpers for RejectReason {
    fn id(&self) -> usize {
        self.id
    }
}