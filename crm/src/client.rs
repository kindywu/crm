#![allow(unused)]

use anyhow::Result;
use crm::{user_client::UserClient, CreateUserRequest, GetUserRequest};
use tonic::Request;

const CRM_SERVER: &str = "http://[::1]:50000";

#[tokio::main]
async fn main() -> Result<()> {
    call_user_service().await?;
    Ok(())
}

async fn call_user_service() -> Result<()> {
    let mut client = UserClient::connect(CRM_SERVER).await?;

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
