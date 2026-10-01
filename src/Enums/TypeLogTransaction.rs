#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub enum TypeLogTransaction{
    FRAUD,
    METRICS,
    ACCOUNT_LIMIT,
    RECURRENCY_LIMIT
}