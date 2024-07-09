use anyhow::Result;
use crm::{
    user_service_server::{UserService, UserServiceServer},
    AppConfig, CreateUserRequest, GetUserRequest, User,
};
use tonic::{transport::Server, Request, Response, Status};

#[derive(Default)]
pub struct UserServer {}

#[tonic::async_trait]
impl UserService for UserServer {
    async fn get_user(&self, request: Request<GetUserRequest>) -> Result<Response<User>, Status> {
        let input = request.into_inner();
        println!("get_user: {:?}", input);
        Ok(Response::new(User::default()))
    }
    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<User>, Status> {
        let input = request.into_inner();
        println!("create_user: {:?}", input);
        Ok(Response::new(User::default()))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = AppConfig::load()?;
    let addr = format!("[::1]:{}", config.server.port).parse().unwrap();
    let svc = UserServer::default();

    println!("CrmServer listening on {}", addr);

    Server::builder()
        .add_service(UserServiceServer::new(svc))
        .serve(addr)
        .await?;

    Ok(())
}
