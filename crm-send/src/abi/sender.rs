use anyhow::Result;
use tokio::time::sleep;
use tokio::time::Duration;

use crate::EmailMessage;
use crate::InAppMessage;
use crate::SmsMessage;

#[allow(unused)]
pub trait Sender {
    async fn send(&self) -> Result<String>;
}

impl Sender for EmailMessage {
    async fn send(&self) -> Result<String> {
        println!("send {self:?}");
        sleep(Duration::from_secs(1)).await;
        Ok(self.message_id.clone())
    }
}

impl Sender for InAppMessage {
    async fn send(&self) -> Result<String> {
        println!("send {self:?}");
        sleep(Duration::from_secs(1)).await;
        Ok(self.message_id.clone())
    }
}

impl Sender for SmsMessage {
    async fn send(&self) -> Result<String> {
        println!("send {self:?}");
        sleep(Duration::from_secs(1)).await;
        Ok(self.message_id.clone())
    }
}
