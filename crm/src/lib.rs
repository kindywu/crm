mod abi;
mod config;
mod pb;

pub use config::*;
use crm_server::{Crm, CrmServer};
pub use pb::*;

use anyhow::Result;
use crm_metadata::metadata_client::MetadataClient;
use crm_send::notification_client::NotificationClient;
use tonic::{async_trait, transport::Channel, Request, Response, Status};
use user_server::{User, UserServer};
use user_stat::user_stat_client::UserStatClient;

#[derive(Default)]
pub struct UserService {}

impl UserService {
    pub fn into_server(self) -> UserServer<Self> {
        UserServer::new(self)
    }
}

#[tonic::async_trait]
impl User for UserService {
    async fn get_user(
        &self,
        request: Request<GetUserRequest>,
    ) -> Result<Response<UserInfo>, Status> {
        let input = request.into_inner();
        println!("get_user: {:?}", input);
        Ok(Response::new(UserInfo::default()))
    }
    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<UserInfo>, Status> {
        let input = request.into_inner();
        println!("create_user: {:?}", input);
        Ok(Response::new(UserInfo::default()))
    }
}

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
