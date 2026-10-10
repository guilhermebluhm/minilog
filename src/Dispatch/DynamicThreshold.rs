use crate::enums::LogLevel::LogLevel;
use crate::logger::LoggerCore::LoggerLogic;
use crate::Models::Aggregator::EventListener;
use crate::utils::SysData::retrieve_sys_information;

pub fn avaliable_branch_status(logs: &mut Vec<Box<dyn LoggerLogic>>){
    
    let cpu_ram_uses = retrieve_sys_information();
    //falta definir as combinações, para criar os estados baseado inicialmente apenas
    //no estado do sistema
    
    for i in logs.iter_mut() {
        i.change_log_level(LogLevel::Error);
    }

}