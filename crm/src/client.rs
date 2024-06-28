use anyhow::Result;
use crm::{user_service_client::UserServiceClient, CreateUserRequest, GetUserRequest};
use tonic::Request;

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = UserServiceClient::connect("http://[::1]:50051").await?;

    let request = Request::new(GetUserRequest { id: 1 });

    let response = client.get_user(request).await?;

    println!("RESPONSE={:?}", response);

    let request = Request::new(CreateUserRequest {
        name: "kindy".to_string(),
        email: "kindywu@outlook.com".to_string(),
    });

    let response = client.create_user(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
