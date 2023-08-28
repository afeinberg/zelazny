use anyhow::Result;

use protos::tablet::tablet_client::TabletClient;
use protos::StatusCheckRequest;

use tonic::{transport, Request};

pub struct Client {
    grpc_client: TabletClient<transport::Channel>,
}

impl Client {
    pub async fn new(url: String) -> Result<Self> {
        let grpc_client = TabletClient::connect(url).await?;
        Ok(Self { grpc_client })
    }

    pub async fn status_check(&mut self) -> Result<String> {
        let response = self
            .grpc_client
            .status_check(Request::new(StatusCheckRequest {
                id: "table_client".into(),
            }))
            .await?;
        let response = response.into_inner();
        Ok(response.id)
    }
}
