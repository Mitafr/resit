use std::path::PathBuf;

use resit::api::PesitSession;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let mut client = PesitSession::connect("127.0.0.1:9003").await?;

    for _i in 0..1 {
        let path = PathBuf::from("test_file.txt");
        client.send_file(&path).await?;
    }
    client.disconnect().await?;

    Ok(())
}
