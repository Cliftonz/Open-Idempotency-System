use std::error::Error;
use std::time::Duration;
use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use crate::open_idempotency::MessageStatus;

#[async_trait]
pub trait IDatabase {
    async fn exists(&mut self, key: String, app_id: String) -> Result<IdempotencyTransaction, Box<dyn Error + Send + Sync>>;
    async fn delete (&mut self, key: String, app_id: String) -> Result<(), Box<dyn Error + Send + Sync>>;
    async fn put (&mut self, key: String, app_id: String, value: IdempotencyTransaction, ttl: Option<Duration>) -> Result<(), Box<dyn Error + Send + Sync>>;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IdempotencyTransaction {
    pub status: MessageStatusDef,
    pub response: String
}

impl IdempotencyTransaction {
    pub fn new_from_status(status: MessageStatusDef) -> IdempotencyTransaction {
        IdempotencyTransaction {
            status,
            response: String::from("")
        }
    }

    pub fn new_default_none() -> IdempotencyTransaction {
        IdempotencyTransaction {
            status: MessageStatusDef::None,
            response: String::from("")
        }
    }

    pub fn new_default_in_progress() -> IdempotencyTransaction {
        IdempotencyTransaction {
            status: MessageStatusDef::InProgress,
            response: String::from("")
        }
    }
    pub fn new(status: MessageStatusDef, response: String) -> IdempotencyTransaction {
        IdempotencyTransaction {
            status,
            response
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum MessageStatusDef {
    None = 0,
    InProgress = 1,
    Completed = 2,
}
impl MessageStatusDef {
    pub fn map_to_grpc(&self) -> MessageStatus {
        match self {
            MessageStatusDef::None => { MessageStatus::None },
            MessageStatusDef::Completed => { MessageStatus::Completed },
            MessageStatusDef::InProgress => { MessageStatus::InProgress }
        }
    }
}



pub fn combine_key(key: String, app_id: String) -> String {
    let mut full_key = app_id.clone();
    full_key.push_str(":");
    full_key.push_str(&key[..]);
    full_key
}
