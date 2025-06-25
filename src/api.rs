use crate::client::PesitClient;
use crate::error::PesitError;
use std::path::Path;

pub struct PesitSession {
    client: PesitClient,
    addr: String,
}

impl PesitSession {
    pub async fn connect(addr: &str) -> Result<Self, PesitError> {
        let client = PesitClient::connect(addr).await?;
        Ok(Self {
            client,
            addr: addr.to_string(),
        })
    }

    pub async fn disconnect(&mut self) -> Result<(), PesitError> {
        self.client.disconnect().await?;
        Ok(())
    }

    pub async fn send_file(&mut self, _path: &Path) -> Result<(), PesitError> {
        self.client.init_connection().await?;
        Ok(())
    }

    pub fn addr(&self) -> &str {
        &self.addr
    }
}
