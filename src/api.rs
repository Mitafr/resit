use crate::client::PesitClient;
use crate::error::PesitError;
use std::path::Path;

/// Represents a PESIT session.
#[derive(Debug)]
pub struct PesitSession {
    client: PesitClient,
    addr: String,
}

impl PesitSession {
    /// Connects to the server.
    ///
    /// This will establish a connection to the server at the given address.
    ///
    /// # Errors
    /// Returns an error if the connection fails.
    pub async fn connect(addr: &str) -> Result<Self, PesitError> {
        let client = PesitClient::connect(addr).await?;
        Ok(Self {
            client,
            addr: addr.to_string(),
        })
    }

    /// Disconnects from the server.
    ///
    /// This will close the connection to the server.
    ///
    /// # Errors
    /// Returns an error if the disconnection fails.
    pub async fn disconnect(&mut self) -> Result<(), PesitError> {
        self.client.disconnect().await?;
        Ok(())
    }

    /// Sends a file to the server.
    ///
    /// # Errors
    ///
    /// Returns an error if the file could not be sent.
    pub async fn send_file<P: AsRef<Path>>(&mut self, path: P) -> Result<(), PesitError> {
        self.client.init_connection().await?;
        Ok(())
    }

    /// Returns the address of the server.
    pub fn addr(&self) -> &str {
        &self.addr
    }
}
