use anyhow::Result;
use crm_metadata::metadata_client::MetadataClient;
use crm_send::notification_client::NotificationClient;
use tonic::{async_trait, transport::Channel, Request, Response, Status};
use user_stat::user_stat_client::UserStatClient;

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
        _request: Request<WelcomeRequest>,
    ) -> Result<Response<WelcomeResponse>, Status> {
        todo!()
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
