#![allow(unused)]

use anyhow::Result;
use crm::{
    crm_client::CrmClient, user_client::UserClient, CreateUserRequest, GetUserRequest,
    WelcomeRequest,
};
use tonic::Request;
use uuid::Uuid;

const CRM_SERVER: &str = "http://[::1]:50000";

#[tokio::main]
async fn main() -> Result<()> {
    // call_user_service().await?;
    call_crm_welcome().await?;
    Ok(())
}

async fn call_user_service() -> Result<()> {
    let mut client = UserClient::connect(CRM_SERVER).await?;

    let request = Request::new(GetUserRequest { id: 1 });

    let response = client.get_user(request).await?;

    println!("get_user response: {:?}", response);

    let request = Request::new(CreateUserRequest {
        name: "kindy".to_string(),
        email: "kindywu@outlook.com".to_string(),
    });

    let response = client.create_user(request).await?;

    println!("create_user response: {:?}", response);
    Ok(())
}

async fn call_crm_welcome() -> Result<()> {
    let mut client = CrmClient::connect("http://[::1]:50000").await?;

    let req = WelcomeRequest {
        id: Uuid::new_v4().to_string(),
        interval: 99,
        content_ids: [1, 2, 3].to_vec(),
    };

    let response = client.welcome(Request::new(req)).await?.into_inner();
    println!("welcome response: {:?}", response);
    Ok(())
}
