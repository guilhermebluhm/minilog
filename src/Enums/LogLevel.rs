use serde::Serialize;

#[derive(Debug, PartialOrd, PartialEq, Clone, Serialize)]
pub enum LogLevel{
    Info,
    Warn,
    Error,
    Fatal
}