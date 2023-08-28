use anyhow::Result;

use tablet::Client;

#[tokio::main]
async fn main() -> Result<()> {
    let tablet_grpc_addr = std::env::args().nth(1).expect("no grpc address given");
    println!("Connecting to {tablet_grpc_addr}");
    let mut client = Client::new(tablet_grpc_addr).await?;
    println!("Connected");
    let remote_id = client.status_check().await?;
    println!("Status check OK, received id {remote_id}");
    Ok(())
}
