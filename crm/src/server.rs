use anyhow::Result;
use crm::{
    user_service_server::{UserService, UserServiceServer},
    CreateUserRequest, GetUserRequest, User,
};
use tonic::{transport::Server, Request, Response, Status};

#[derive(Default)]
pub struct UserServer {}

#[tonic::async_trait]
impl UserService for UserServer {
    async fn get_user(&self, _request: Request<GetUserRequest>) -> Result<Response<User>, Status> {
        Ok(Response::new(User::default()))
    }
    async fn create_user(
        &self,
        _request: Request<CreateUserRequest>,
    ) -> Result<Response<User>, Status> {
        Ok(Response::new(User::default()))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "[::1]:50051".parse().unwrap();
    let user_server = UserServer::default();

    println!("UserServer listening on {}", addr);

    Server::builder()
        .add_service(UserServiceServer::new(user_server))
        .serve(addr)
        .await?;

    Ok(())
}
