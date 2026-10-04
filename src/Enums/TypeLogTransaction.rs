use serde::Serialize;

#[derive(Clone, Debug, PartialOrd, PartialEq, Serialize)]
pub enum TypeLogTransaction{
    FRAUD,
    METRICS,
    ACCOUNT_LIMIT,
    RECURRENCY_LIMIT
}