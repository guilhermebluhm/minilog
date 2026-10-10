use crate::Models::CreateBy::CreateBy;
use crate::Models::RejectReason::RejectReason;
use crate::enums::ClientTier::ClientTier;
use crate::enums::LogLevel::LogLevel;
use crate::logger::LoggerCore::{Logger, LoggerLogic};
use serde::Serialize;
use crate::enums::SeverityLevel::SeverityLevel;

#[derive(Debug, Clone, Serialize, )]
pub struct Transaction {
    pub id: u32,
    pub amount: f64,
    pub account_id: u32, //identificador do numero da conta
    pub rejection_reasons: Vec<RejectReason>, //lista de razoes para rejeicao
    pub client_tier: ClientTier,
    pub created_by: CreateBy,
    pub LogTransaction: Logger //retirar o severity e passar as acoes de logger para dentro do LoggerCore
}

impl Transaction {
    pub fn new(id: u32, amount: f64, account_id: u32, tier: ClientTier) -> Transaction {
        Self{
            id, amount, account_id,
            rejection_reasons: vec![],
            client_tier: tier,
            created_by: CreateBy::new(),
            LogTransaction: Logger{
                Log: LogLevel::Info,
                severity: SeverityLevel::INFORMATIVE}
        }
    }

    pub fn add_rejection(&mut self, reason: RejectReason) {
        self.rejection_reasons.push(reason);
    }
}

impl LoggerLogic for Transaction {
    fn log_level(&self) -> LogLevel {
        self.LogTransaction.Log.clone()
    }

    fn actual_severity_level(&self) -> SeverityLevel {
        self.LogTransaction.severity.clone()
    }

    fn change_severity_level(&mut self, severity_level: SeverityLevel) {
        self.LogTransaction.severity = severity_level;
    }

    fn change_log_level(&mut self, new_log_level: LogLevel) {
        self.LogTransaction.Log = new_log_level;
    }

    fn transaction_id(&self) -> u32 {
        self.id
    }
}