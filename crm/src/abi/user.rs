use crate::{
    user_server::{User, UserServer},
    CreateUserRequest, GetUserRequest, UserInfo,
};
use anyhow::Result;
use prost_types::Timestamp;
use std::time::SystemTime;
use tonic::{service::interceptor::InterceptedService, Request, Response, Status};
use tracing::info;

use super::AuthInterceptor;

impl UserInfo {
    pub fn new(id: u64, name: &str, email: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            email: email.to_string(),
            created_at: Some(Timestamp::from(SystemTime::now())),
        }
    }
}

#[derive(Default)]
pub struct UserService {}

impl UserService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn into_server(
        self,
    ) -> Result<InterceptedService<UserServer<UserService>, AuthInterceptor>> {
        Ok(UserServer::with_interceptor(self, AuthInterceptor))
    }
}

#[tonic::async_trait]
impl User for UserService {
    async fn get_user(
        &self,
        request: Request<GetUserRequest>,
    ) -> Result<Response<UserInfo>, Status> {
        let input = request.into_inner();
        info!("get_user: {:?}", input);
        Ok(Response::new(UserInfo::default()))
    }
    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<UserInfo>, Status> {
        let input = request.into_inner();
        info!("create_user: {:?}", input);
        Ok(Response::new(UserInfo::default()))
    }
}
