use std::time::SystemTime;

use prost_types::Timestamp;

use crate::UserInfo;

impl UserInfo {
    pub fn new(id: u64, name: &str, email: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            email: email.to_string(),
            created_at: Some(Timestamp::from(SystemTime::now())),
        }
    }
}
