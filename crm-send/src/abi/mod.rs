use std::pin::Pin;

use tokio_stream::Stream;
use tonic::{async_trait, Request, Response, Status, Streaming};

use crate::{notification_server::Notification, AppConfig, SendRequest, SendResponse};

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
        _request: Request<Streaming<SendRequest>>,
    ) -> ServiceResult<ResponseStream> {
        todo!()
    }
}
