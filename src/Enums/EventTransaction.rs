use crate::enums::SeverityLevel::SeverityLevel;

#[derive(Debug, PartialOrd, PartialEq)]
pub enum EventTransaction{
    Continue(SeverityLevel),
    Blocked(SeverityLevel),
}