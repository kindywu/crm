mod metadata;

use tonic::{async_trait, Request, Streaming};

use crate::pb::{metadata_server::Metadata, MaterializeRequest};

pub use metadata::*;

#[async_trait]
impl Metadata for MetadataService {
    type MaterializeStream = ResponseStream;

    async fn materialize(
        &self,
        request: Request<Streaming<MaterializeRequest>>,
    ) -> ServiceResult<Self::MaterializeStream> {
        self.materialize(request.into_inner()).await
    }
}
