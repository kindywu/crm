#![allow(unused)]

use anyhow::Result;
use crm::{
    crm_client::CrmClient, user_client::UserClient, CreateUserRequest, GetUserRequest,
    RecallRequest, RemindRequest, WelcomeRequest,
};
use tonic::Request;
use uuid::Uuid;

const CRM_SERVER: &str = "http://[::1]:50000";

#[tokio::main]
async fn main() -> Result<()> {
    // call_user_service().await?;
    // call_crm_welcome().await?;
    // call_crm_recall().await?;
    call_crm_remind().await?;
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
    let mut client = CrmClient::connect(CRM_SERVER).await?;

    let req = WelcomeRequest {
        id: Uuid::new_v4().to_string(),
        interval: 99,
        content_ids: [1, 2, 3].to_vec(),
    };

    let response = client.welcome(Request::new(req)).await?.into_inner();
    println!("welcome response: {:?}", response);
    Ok(())
}

async fn call_crm_recall() -> Result<()> {
    let mut client = CrmClient::connect(CRM_SERVER).await?;

    let req = RecallRequest {
        id: Uuid::new_v4().to_string(),
        last_visit_interval: 10, //测试数据：SELECT email, name, last_visited_at FROM user_stats WHERE last_visited_at > last_email_notification order by last_visited_at desc limit 10;
        content_ids: [1, 2, 3].to_vec(),
    };

    let response = client.recall(Request::new(req)).await?.into_inner();
    println!("recall response: {:?}", response);
    Ok(())
}

async fn call_crm_remind() -> Result<()> {
    let mut client = CrmClient::connect(CRM_SERVER).await?;

    let req = RemindRequest {
        id: Uuid::new_v4().to_string(),
        last_visit_interval: 10, //测试数据：SELECT email, name, last_visited_at FROM user_stats WHERE last_visited_at > last_email_notification order by last_visited_at desc limit 10;
    };

    let response = client.remind(Request::new(req)).await?.into_inner();
    println!("recall response: {:?}", response);
    Ok(())
}
