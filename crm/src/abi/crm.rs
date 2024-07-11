use std::collections::HashSet;

use anyhow::Result;
use crm_metadata::{metadata_client::MetadataClient, Content, MaterializeRequest};
use crm_send::{notification_client::NotificationClient, EmailMessage, SendRequest};
// use tokio_stream::StreamExt;
use futures::StreamExt;
use tonic::{
    async_trait, transport::Channel, IntoStreamingRequest, Request, Response, Status, Streaming,
};
use user_stat::{user_stat_client::UserStatClient, User};
use uuid::Uuid;

use crate::{
    crm_server::{Crm, CrmServer},
    AppConfig, RecallRequest, RecallResponse, RemindRequest, RemindResponse, WelcomeRequest,
    WelcomeResponse,
};

use super::build_query::{
    new_query_request_for_wellcome, new_raw_query_request_for_recall,
    new_raw_query_request_for_remind,
};

#[derive(Debug)]
#[allow(unused)]
pub struct CrmService {
    config: AppConfig,
    user_stats: UserStatClient<Channel>,
    notification: NotificationClient<Channel>,
    metadata: MetadataClient<Channel>,
}

impl CrmService {
    pub async fn try_new() -> Result<Self> {
        let config = AppConfig::load()?;
        let server = config.server.clone();
        let user_stats = UserStatClient::connect(server.user_stats).await?;
        let notification = NotificationClient::connect(server.notification).await?;
        let metadata = MetadataClient::connect(server.metadata).await?;

        Ok(Self {
            config,
            user_stats,
            notification,
            metadata,
        })
    }

    pub fn into_server(self) -> CrmServer<Self> {
        CrmServer::new(self)
    }
}

#[async_trait]
impl Crm for CrmService {
    async fn welcome(
        &self,
        request: Request<WelcomeRequest>,
    ) -> Result<Response<WelcomeResponse>, Status> {
        let req = request.into_inner();
        let query_request = new_query_request_for_wellcome(req.interval as _);
        let query_response = self.user_stats.clone().query(query_request).await?;
        let query_stream = query_response.into_inner();

        let sender = self.config.server.sender_email.clone();
        let contents = self.get_contents(req.content_ids).await?;

        let send_request_stream = CrmService::build_send_request_stream(
            "Welcome".to_string(),
            query_stream,
            sender,
            contents,
        );

        self.notification.clone().send(send_request_stream).await?;

        Ok(Response::new(WelcomeResponse { id: req.id }))
    }

    async fn recall(
        &self,
        request: Request<RecallRequest>,
    ) -> Result<Response<RecallResponse>, Status> {
        let req = request.into_inner();

        let query_request = new_raw_query_request_for_recall(req.last_visit_interval as _);
        let query_response = self.user_stats.clone().raw_query(query_request).await?;
        let query_stream = query_response.into_inner();

        let sender = self.config.server.sender_email.clone();
        let contents = self.get_contents(req.content_ids).await?;

        let send_request_stream = CrmService::build_send_request_stream(
            "Recall".to_string(),
            query_stream,
            sender,
            contents,
        );

        self.notification.clone().send(send_request_stream).await?;

        Ok(Response::new(RecallResponse { id: req.id }))
    }

    async fn remind(
        &self,
        request: Request<RemindRequest>,
    ) -> Result<Response<RemindResponse>, Status> {
        let req = request.into_inner();

        let query_request = new_raw_query_request_for_remind(req.last_visit_interval as _);
        let query_response = self.user_stats.clone().raw_query(query_request).await?;
        let query_stream = query_response.into_inner();

        let sender = self.config.server.sender_email.clone();

        let send_request_stream = CrmService::build_send_request_stream_for_remind(
            self.metadata.clone(),
            "Remind".to_string(),
            query_stream,
            sender,
        );

        self.notification.clone().send(send_request_stream).await?;

        Ok(Response::new(RemindResponse { id: req.id }))
    }
}

impl CrmService {
    async fn get_contents(&self, content_ids: Vec<u32>) -> Result<Vec<Content>, Status> {
        let content_ids: Vec<_> = content_ids
            .into_iter()
            .map(|id| MaterializeRequest { id })
            .collect();
        let stream = tokio_stream::iter(content_ids);
        let metadata_response = self.metadata.clone().materialize(stream).await?;
        let metadata_stream = metadata_response.into_inner();
        let contents: Vec<Content> = metadata_stream
            .filter_map(|item| async move { item.ok() })
            .collect::<Vec<Content>>()
            .await;
        Ok(contents)
    }

    fn build_send_request_stream(
        subject: String,
        query_stream: Streaming<User>,
        sender: String,
        contents: Vec<Content>,
    ) -> impl IntoStreamingRequest<Message = SendRequest> {
        query_stream.filter_map(move |v| {
            let subject = subject.clone();
            let sender = sender.clone();
            let contents = contents.clone();
            async move {
                let v = v.ok()?;
                Some(gen_send_req(subject, sender, v.email, contents))
            }
        })
    }

    fn build_send_request_stream_for_remind(
        metadata: MetadataClient<Channel>,
        subject: String,
        query_stream: Streaming<User>,
        sender: String,
    ) -> impl IntoStreamingRequest<Message = SendRequest> {
        query_stream.filter_map(move |v| {
            let mut metadata = metadata.clone();
            let subject = subject.clone();
            let sender = sender.clone();

            async move {
                let user = v.ok()?;

                let content_ids = user
                    .viewed_but_not_starteds
                    .into_iter()
                    .chain(user.started_but_not_finisheds)
                    .collect::<HashSet<_>>(); //去重

                // 快速返回
                if content_ids.is_empty() {
                    return None;
                }

                println!("getting content for send: {content_ids:?}");

                let content_ids: Vec<_> = content_ids
                    .into_iter()
                    .map(|id| MaterializeRequest { id: id as _ })
                    .collect();

                let stream = tokio_stream::iter(content_ids);
                if let Ok(metadata_response) = metadata.materialize(stream).await {
                    let metadata_stream = metadata_response.into_inner();
                    let contents: Vec<Content> = metadata_stream
                        .filter_map(|item| async move { item.ok() })
                        .collect::<Vec<Content>>()
                        .await;

                    Some(gen_send_req(subject, sender, user.email, contents))
                } else {
                    None
                }
            }
        })
    }
}

fn gen_send_req(
    subject: String,
    sender: String,
    recipient: String,
    contents: Vec<Content>,
) -> SendRequest {
    EmailMessage {
        message_id: Uuid::new_v4().to_string(),
        sender,
        recipients: vec![recipient],
        subject,
        body: format!("Tpl: {:?}", contents),
    }
    .into()
}
