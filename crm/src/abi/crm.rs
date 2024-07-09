use std::sync::Arc;

use anyhow::Result;
use chrono::{Duration, Utc};
use crm_metadata::{metadata_client::MetadataClient, Content, MaterializeRequest};
use crm_send::{notification_client::NotificationClient, EmailMessage};
use prost_types::Timestamp;
use tokio_stream::StreamExt;
use tonic::{async_trait, transport::Channel, Request, Response, Status};
use user_stat::{user_stat_client::UserStatClient, QueryRequest, QueryRequestBuilder, TimeQuery};
use uuid::Uuid;

use crate::{
    crm_server::{Crm, CrmServer},
    AppConfig, RecallRequest, RecallResponse, RemindRequest, RemindResponse, WelcomeRequest,
    WelcomeResponse,
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
        let query_request = new_query_request_with_timestamp("created_at", req.interval as _);
        let query_response = self.user_stats.clone().query(query_request).await?;
        let query_stream = query_response.into_inner();

        let content_ids: Vec<_> = req
            .content_ids
            .into_iter()
            .map(|id| MaterializeRequest { id })
            .collect();
        let stream = tokio_stream::iter(content_ids);
        let metadata_response = self.metadata.clone().materialize(stream).await?;
        let metadata_stream = metadata_response.into_inner();
        let contents: Vec<Content> = metadata_stream
            .filter_map(|item| item.ok())
            .collect::<Vec<Content>>()
            .await;

        let sender = self.config.server.sender_email.clone();
        let contents = Arc::new(contents);

        let send_request_stream = query_stream.filter_map(move |u| {
            let sender = sender.clone();
            let contents = contents.clone();
            match u {
                Ok(u) => {
                    let s = Some(
                        EmailMessage {
                            message_id: Uuid::new_v4().to_string(),
                            sender,
                            recipients: vec![u.email],
                            subject: "Welcome".to_string(),
                            body: format!("Tpl: {:?}", contents),
                        }
                        .into(),
                    );
                    println!("{s:?}");
                    s
                }
                Err(_) => None,
            }
        });

        self.notification.clone().send(send_request_stream).await?;

        Ok(Response::new(WelcomeResponse { id: req.id }))
    }

    async fn recall(
        &self,
        _request: Request<RecallRequest>,
    ) -> Result<Response<RecallResponse>, Status> {
        todo!()
    }

    async fn remind(
        &self,
        _request: Request<RemindRequest>,
    ) -> Result<Response<RemindResponse>, Status> {
        todo!()
    }
}

fn new_query_request_with_timestamp(name: impl Into<String>, interval: i64) -> QueryRequest {
    let d1 = Utc::now() - Duration::days(interval);
    let d2 = d1 + Duration::days(1);

    let d1 = Timestamp {
        seconds: d1.timestamp(),
        nanos: 0,
    };

    let d2 = Timestamp {
        seconds: d2.timestamp(),
        nanos: 0,
    };

    let query = QueryRequestBuilder::default()
        .timestamp((
            name.into(),
            TimeQuery {
                lower: Some(d1),
                upper: Some(d2),
            },
        ))
        .build()
        .expect("Failed to build query request");
    query
}
