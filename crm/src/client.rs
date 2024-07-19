#![allow(unused)]

use anyhow::Result;
use crm::{
    crm_client::CrmClient, user_client::UserClient, CreateUserRequest, GetUserRequest,
    RecallRequest, RemindRequest, WelcomeRequest,
};
use tonic::{
    metadata::MetadataValue,
    service::interceptor::InterceptedService,
    transport::{Certificate, Channel, ClientTlsConfig},
    Request, Status,
};
use uuid::Uuid;

const CRM_SERVER: &str = "https://[::1]:50000";

#[tokio::main]
async fn main() -> Result<()> {
    let pem = include_str!("../fixtures/rootCA.pem");
    let tls = ClientTlsConfig::new()
        .ca_certificate(Certificate::from_pem(pem))
        .domain_name("localhost");

    let channel = Channel::from_static(CRM_SERVER)
        .tls_config(tls)?
        .connect()
        .await?;

    let token: MetadataValue<_> = "abc".parse()?;
    let mut client = UserClient::with_interceptor(channel.clone(), move |mut req: Request<()>| {
        req.metadata_mut().insert("authorization", token.clone());
        Ok(req)
    });
    call_user_service(&mut client).await?;

    let token: MetadataValue<_> = "abc".parse()?;
    let mut client = CrmClient::with_interceptor(channel.clone(), move |mut req: Request<()>| {
        req.metadata_mut().insert("authorization", token.clone());
        Ok(req)
    });
    call_crm_welcome(&mut client).await?;
    call_crm_recall(&mut client).await?;
    call_crm_remind(&mut client).await?;
    Ok(())
}

async fn call_user_service(
    client: &mut UserClient<
        InterceptedService<Channel, impl Fn(Request<()>) -> Result<Request<()>, Status>>,
    >,
) -> Result<()> {
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

async fn call_crm_welcome(
    client: &mut CrmClient<
        InterceptedService<Channel, impl Fn(Request<()>) -> Result<Request<()>, Status>>,
    >,
) -> Result<()> {
    let req = WelcomeRequest {
        id: Uuid::new_v4().to_string(),
        interval: 108,
        content_ids: [1, 2, 3].to_vec(),
    };

    let response = client.welcome(Request::new(req)).await?.into_inner();
    println!("welcome response: {:?}", response);
    Ok(())
}

async fn call_crm_recall(
    client: &mut CrmClient<
        InterceptedService<Channel, impl Fn(Request<()>) -> Result<Request<()>, Status>>,
    >,
) -> Result<()> {
    let req = RecallRequest {
        id: Uuid::new_v4().to_string(),
        last_visit_interval: 19, //测试数据：SELECT email, name, last_visited_at FROM user_stats WHERE last_visited_at > last_email_notification order by last_visited_at desc limit 10;
        content_ids: [1, 2, 3].to_vec(),
    };

    let response = client.recall(Request::new(req)).await?.into_inner();
    println!("recall response: {:?}", response);
    Ok(())
}

async fn call_crm_remind(
    client: &mut CrmClient<
        InterceptedService<Channel, impl Fn(Request<()>) -> Result<Request<()>, Status>>,
    >,
) -> Result<()> {
    let req = RemindRequest {
        id: Uuid::new_v4().to_string(),
        last_visit_interval: 10, //测试数据：SELECT email, name, last_visited_at FROM user_stats WHERE last_visited_at > last_email_notification order by last_visited_at desc limit 10;
    };

    let response = client.remind(Request::new(req)).await?.into_inner();
    println!("recall response: {:?}", response);
    Ok(())
}
