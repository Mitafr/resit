use resit::server::PesitServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let mut server = PesitServer::new(9003).await?;
    server.run().await?;
    Ok(())
}
