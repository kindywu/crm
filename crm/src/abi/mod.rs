mod build_query;
mod crm;
mod user;

pub use crm::*;
use tonic::{service::Interceptor, Request, Status};
use tracing::info;
pub use user::UserService;

#[derive(Clone)]
pub struct AuthInterceptor;

impl Interceptor for AuthInterceptor {
    fn call(&mut self, mut request: Request<()>) -> Result<Request<()>, Status> {
        let token = request
            .metadata()
            .get("authorization")
            .and_then(|v| v.to_str().ok());
        info!("token: {:?}", token);
        if Some("abc") == token {
            request.extensions_mut().insert("kindy");
            Ok(request)
        } else {
            Err(Status::unauthenticated(format!(
                "unauthenticated: {token:?}"
            )))
        }
    }
}
