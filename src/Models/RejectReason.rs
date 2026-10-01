use crate::enums::EventTransaction::EventTransaction;
use crate::enums::TypeLogTransaction::TypeLogTransaction;

#[derive(Debug, Clone)]
pub struct RejectReason {
    pub id: usize,
    pub reason: String,
    pub type_reject: TypeLogTransaction
}

trait Helpers{
    fn id(&self) -> usize;
    fn type_reject(&self) -> TypeLogTransaction;
}

impl Helpers for RejectReason {
    fn id(&self) -> usize {
        self.id
    }

    fn type_reject(&self) -> TypeLogTransaction {
        self.type_reject.clone()
    }
}