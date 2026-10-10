use serde::Serialize;
use crate::enums::LogLevel::LogLevel;
use crate::Models::Transaction::Transaction;

#[derive(Debug, Clone, Serialize)]
pub struct Logger{
    pub Log: LogLevel
}

pub trait LoggerLogic{
    fn log_level(&self) -> LogLevel;
    fn change_log_level(&mut self, new_log_level: LogLevel);
    fn transaction_id(&self) -> u32;
    fn minimal_log_level(&self) -> LogLevel {
        LogLevel::Error
    }

}