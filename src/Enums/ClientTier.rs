use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub enum ClientTier{
    BASIC,
    SILVER,
    GOLD,
    PLATINUM
}