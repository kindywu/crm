use std::time::SystemTime;
use tonic::{Request, Response, Status};

use crate::{
    user_server::{User, UserServer},
    CreateUserRequest, GetUserRequest, UserInfo,
};
use prost_types::Timestamp;

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
