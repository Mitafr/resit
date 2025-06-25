use crate::{error::PesitError, protocol::codec::PesitCodec};
use tokio::net::TcpStream;
use tokio_util::codec::Framed;

pub type PesitFramedStream = Framed<TcpStream, PesitCodec>;

pub async fn connect(addr: &str) -> Result<PesitFramedStream, PesitError> {
    let stream = TcpStream::connect(addr).await?;
    Ok(Framed::new(stream, PesitCodec))
}
