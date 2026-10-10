use std::thread::{spawn, JoinHandle};
use crate::enums::SeverityLevel::SeverityLevel;
use crate::logger::LoggerCore::LoggerLogic;

pub fn generate_notification(event_transaction: &Vec<Box<dyn LoggerLogic>>){

    let event_for_notification = event_transaction
        .iter()
        .filter(|f| f.actual_severity_level() >= minimal_severity_for_notification())
        .collect::<Vec<&Box<dyn LoggerLogic>>>();

    for i in 0..event_for_notification.len() {

        let id = event_transaction.get(i).unwrap().transaction_id();
        let _ = spawn(move ||{
            println!("NOTIFICATION FOR TRANSACTION ID: {}", id);
        });

        std::thread::sleep(std::time::Duration::from_millis(100));

    }

}

fn minimal_severity_for_notification() -> SeverityLevel {
    SeverityLevel::INFORMATIVE
}