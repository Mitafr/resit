use bb8;

use crate::PesitSession;

pub struct PesitPool {
    client: PesitSession,
}

impl bb8::ManageConnection for PesitPool {
    type Connection = PesitSession;
    type Error = crate::error::PesitError;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        let addr = self.client.addr();
        PesitSession::connect(addr).await
    }

    async fn is_valid(&self, _conn: &mut Self::Connection) -> Result<(), Self::Error> {
        // Send ping
        Ok(())
    }

    fn has_broken(&self, _conn: &mut Self::Connection) -> bool {
        false
    }
}
