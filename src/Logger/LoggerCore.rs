use crate::enums::LogLevel::LogLevel;
use crate::enums::SeverityLevel::SeverityLevel;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Logger{
    pub Log: LogLevel,
    pub severity: SeverityLevel
}

pub trait LoggerLogic: Sync + Send {
    fn log_level(&self) -> LogLevel;
    fn actual_severity_level(&self) -> SeverityLevel;
    fn change_severity_level(&mut self, severity_level: SeverityLevel);
    fn change_log_level(&mut self, new_log_level: LogLevel);
    fn transaction_id(&self) -> u32;
    fn minimal_log_level(&self) -> LogLevel {
        LogLevel::Error
    }

}