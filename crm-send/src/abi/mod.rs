mod send;

use std::pin::Pin;

use chrono::Local;
use prost_types::Timestamp;
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, Stream, StreamExt};
use tonic::{async_trait, Request, Response, Status, Streaming};

use crate::{
    notification_server::Notification, send_request::Msg, AppConfig, SendRequest, SendResponse,
};

const CHANNEL_SIZE: usize = 1024;

#[allow(unused)]
pub struct NotificationService {
    config: AppConfig,
}

impl NotificationService {
    pub fn new(config: AppConfig) -> Self {
        Self { config }
    }
}

pub type ServiceResult<T> = Result<Response<T>, Status>;
pub type ResponseStream = Pin<Box<dyn Stream<Item = Result<SendResponse, Status>> + Send>>;

#[async_trait]
impl Notification for NotificationService {
    type SendStream = ResponseStream;

    async fn send(
        &self,
        request: Request<Streaming<SendRequest>>,
    ) -> ServiceResult<ResponseStream> {
        let mut stream = request.into_inner();
        let (tx, rx) = mpsc::channel(CHANNEL_SIZE);
        tokio::spawn(async move {
            while let Some(Ok(req)) = stream.next().await {
                if let Some(msg) = req.msg {
                    let message_id = match msg {
                        Msg::Email(msg) => msg.message_id,
                        Msg::Sms(msg) => msg.message_id,
                        Msg::InApp(msg) => msg.message_id,
                    };

                    let now = Local::now();
                    let timestamp = Some(Timestamp {
                        seconds: now.timestamp(),
                        nanos: now.timestamp_subsec_nanos() as i32,
                    });
                    let res = SendResponse {
                        message_id,
                        timestamp,
                    };
                    tx.send(Ok(res)).await.unwrap();
                }
            }
        });
        let stream = ReceiverStream::new(rx);
        Ok(Response::new(Box::pin(stream)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        pb::{EmailMessage, InAppMessage, SmsMessage},
        AppConfig,
    };
    use anyhow::Result;

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

    #[tokio::test]
    async fn send_should_work() -> Result<()> {
        let config = AppConfig::load()?;
        let service = NotificationService::new(config);
        let events = vec![
            SendRequest {
                msg: Some(Msg::Email(EmailMessage::fake())),
            },
            SendRequest {
                msg: Some(Msg::InApp(InAppMessage::fake())),
            },
            SendRequest {
                msg: Some(Msg::Sms(SmsMessage::fake())),
            },
        ];

        let request = tonic_mock::streaming_request(events);

        // let request = Request::new(stream);
        let response = service.send(request).await?;
        let ret = response.into_inner().collect::<Vec<_>>().await;
        assert_eq!(ret.len(), 3);

        Ok(())
    }
}
