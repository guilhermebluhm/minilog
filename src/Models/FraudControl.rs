use std::fmt;
use std::fmt::{write, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FraudControl(u32);

impl FraudControl{
    pub fn new(id: u32) -> Self {
        Self(id)
    }
}