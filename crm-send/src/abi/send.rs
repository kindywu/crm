use anyhow::Result;
use tokio::time::sleep;
use tokio::time::Duration;

use crate::EmailMessage;
use crate::InAppMessage;
use crate::SmsMessage;

#[allow(unused)]
pub trait Send {
    async fn send(&self) -> Result<String>;
}

impl Send for EmailMessage {
    async fn send(&self) -> Result<String> {
        println!("send {self:?}");
        sleep(Duration::from_secs(1)).await;
        Ok(self.message_id.clone())
    }
}

impl Send for InAppMessage {
    async fn send(&self) -> Result<String> {
        println!("send {self:?}");
        sleep(Duration::from_secs(1)).await;
        Ok(self.message_id.clone())
    }
}

impl Send for SmsMessage {
    async fn send(&self) -> Result<String> {
        println!("send {self:?}");
        sleep(Duration::from_secs(1)).await;
        Ok(self.message_id.clone())
    }
}
