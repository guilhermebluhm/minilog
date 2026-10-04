use std::fs::Metadata;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::Serialize;
use crate::Models::Transaction::Transaction;

#[derive(Clone, Debug, Serialize)]
pub struct CreateBy{
    pub thread_name: String,
    pub metadadata: String,
}

impl CreateBy{
    pub fn new() -> Self {

        Self{
            thread_name: thread_id::get().to_string(),
            metadadata: format!("timestamp: {:?}. file: {:?}. line: {:?}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis(), file!(), line!())
        }


    }
}