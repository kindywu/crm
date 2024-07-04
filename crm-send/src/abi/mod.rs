mod sender;

use std::pin::Pin;

use chrono::Local;
use prost_types::Timestamp;
use sender::Sender;
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, Stream, StreamExt};
use tonic::{async_trait, Request, Response, Status, Streaming};
use tracing::warn;

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
                    let result = match msg {
                        Msg::Email(msg) => msg.send().await,
                        Msg::Sms(msg) => msg.send().await,
                        Msg::InApp(msg) => msg.send().await,
                    };

                    match result {
                        Ok(message_id) => {
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
                        Err(e) => warn!("{}", e),
                    }
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
