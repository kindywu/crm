use anyhow::Result;
use tokio::time::sleep;
use tokio::time::Duration;
use tracing::info;

use crate::EmailMessage;
use crate::InAppMessage;
use crate::SmsMessage;

pub trait Sender {
    async fn send(&self) -> Result<String>;
}

impl Sender for EmailMessage {
    async fn send(&self) -> Result<String> {
        info!("send {self:?}");
        sleep(Duration::from_secs(1)).await;
        Ok(self.message_id.clone())
    }
}

impl Sender for InAppMessage {
    async fn send(&self) -> Result<String> {
        info!("send {self:?}");
        sleep(Duration::from_secs(1)).await;
        Ok(self.message_id.clone())
    }
}

impl Sender for SmsMessage {
    async fn send(&self) -> Result<String> {
        info!("send {self:?}");
        sleep(Duration::from_secs(1)).await;
        Ok(self.message_id.clone())
    }
}

impl EmailMessage {
    pub fn fake() -> Self {
        use fake::faker::internet::en::SafeEmail;
        use fake::Fake;
        use uuid::Uuid;
        EmailMessage {
            message_id: Uuid::new_v4().to_string(),
            sender: SafeEmail().fake(),
            recipients: vec![SafeEmail().fake()],
            subject: "Hello".to_string(),
            body: "Hello, world!".to_string(),
        }
    }
}

#[cfg(test)]
impl InAppMessage {
    pub fn fake() -> Self {
        use uuid::Uuid;
        InAppMessage {
            message_id: Uuid::new_v4().to_string(),
            device_id: Uuid::new_v4().to_string(),
            title: "Hello".to_string(),
            body: "Hello, world!".to_string(),
        }
    }
}

#[cfg(test)]
impl SmsMessage {
    pub fn fake() -> Self {
        use fake::faker::phone_number::en::PhoneNumber;
        use fake::Fake;
        use uuid::Uuid;
        SmsMessage {
            message_id: Uuid::new_v4().to_string(),
            sender: PhoneNumber().fake(),
            recipients: vec![PhoneNumber().fake()],
            body: "Hello, world!".to_string(),
        }
    }
}
