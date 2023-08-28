use tonic::{transport::Server, Request, Response, Status};

use protos::tablet::tablet_server::{Tablet, TabletServer};

use anyhow::Result;
use futures::FutureExt;
use protos::{StatusCheckRequest, StatusCheckResponse};
use tokio::sync::broadcast::Sender;

use crate::Config;

#[derive(Debug, Default)]
pub struct TabletRpcServer {}

#[tonic::async_trait]
impl Tablet for TabletRpcServer {
    async fn status_check(
        &self,
        request: Request<StatusCheckRequest>,
    ) -> Result<Response<StatusCheckResponse>, Status> {
        let request = request.into_inner();
        let reply = StatusCheckResponse {
            id: format!("tablet_server->{}", request.id),
        };
        Ok(Response::new(reply))
    }
}

pub async fn serve(shutdown_tx: Sender<()>, config: Config) -> Result<()> {
    let mut shutdown_rx = shutdown_tx.subscribe();
    let addr = format!("0.0.0.0:{}", config.grpc_port).parse()?;
    let tablet_rpc_server = TabletRpcServer::default();

    Server::builder()
        .add_service(TabletServer::new(tablet_rpc_server))
        .serve_with_shutdown(addr, shutdown_rx.recv().map(|_| ()))
        .await?;

    Ok(())
}
