use std::{future::Future, str::FromStr};

use tokio::net::{unix::SocketAddr, TcpListener, TcpStream, ToSocketAddrs};
use tokio_stream::StreamExt;

use crate::{
    connection::PesitFramedStream,
    error::PesitError,
    protocol::{
        codec::PesitCodec,
        frame::{
            handler::{FConnectHandler, FReleaseHandler},
            types::FrameType,
            Frame,
        },
    },
    state::{ServerState, State},
};

pub(crate) trait FrameHandler<S: State> {
    type Payload;

    fn handle(
        &self,
        conn: &mut PesitFramedStream,
        frame: Frame,
        state: &mut S,
    ) -> impl Future<Output = Result<(), PesitError>> + Send;

    fn extract_payload(&self, frame: &Frame) -> Self::Payload;
}

/// PESIT server struct for handling client connections.
#[derive(Debug)]
pub struct PesitServer {
    addr: String,
    listener: TcpListener,
}

impl PesitServer {
    /// Creates a new PESIT server.
    ///
    /// # Arguments
    /// * `port` - The port number to bind the server to.
    ///
    /// # Errors
    /// This function will return an error if the server fails to bind to the specified port.
    ///
    /// # Example
    /// ```rust
    /// use resit::server::PesitServer;
    /// #[tokio::main]
    /// async fn main() -> Result<(), resit::error::PesitError> {
    ///    let mut server = PesitServer::new("127.0.0.1:8080").await?;
    ///    Ok(())
    /// }
    /// ```
    pub async fn new<A: AsRef<str>>(addr: A) -> Result<Self, PesitError> {
        let listener = match TcpListener::bind(addr.as_ref()).await {
            Ok(tcp_listener) => {
                log::info!("TCP listener started on {}", addr.as_ref());
                tcp_listener
            }
            Err(e) => return Err(e.into()),
        };

        Ok(Self {
            addr: addr.as_ref().into(),
            listener,
        })
    }

    /// Runs the server, accepting incoming connections.
    ///
    /// # Errors
    /// This function will return an error if the server fails to accept a connection.
    /// Or the frame processing fails.
    pub async fn run(&mut self) -> Result<(), PesitError> {
        loop {
            let sock = match self.accept_conn().await {
                Ok(stream) => stream,
                Err(e) => {
                    return Err(e);
                }
            };
            log::info!("Accepted connection from {}", sock.peer_addr()?);

            let resp_command_frame = PesitFramedStream::with_capacity(sock, PesitCodec, 8 * 1024);
            tokio::spawn(async move {
                let mut handler = PesitServerHandler::new(resp_command_frame);
                if let Err(e) = handler.handle().await {
                    log::error!("Failed to handle command: {e}");
                }
            });
        }
    }

    /// Accepts a new incoming connection.
    ///
    /// # Errors
    /// Returns an error if the connection could not be accepted.
    async fn accept_conn(&mut self) -> Result<TcpStream, PesitError> {
        match self.listener.accept().await {
            Ok((sock, _)) => Ok(sock),
            Err(e) => Err(PesitError::from(e)),
        }
    }
}

struct PesitServerHandler {
    conn: PesitFramedStream,
    state: ServerState,
}

impl PesitServerHandler {
    pub fn new(conn: PesitFramedStream) -> Self {
        Self {
            conn,
            state: ServerState::default(),
        }
    }

    pub async fn handle(&mut self) -> Result<(), PesitError> {
        macro_rules! handle_frame {
            ($frame:ident, $handler:ident) => {{
                $handler {}
                    .handle(&mut self.conn, $frame, &mut self.state)
                    .await?;
            }};
        }
        while let Some(frame) = self.conn.next().await {
            match frame {
                Ok(frame) => match frame.header.kind {
                    FrameType::FConnect => handle_frame!(frame, FConnectHandler),
                    FrameType::FRelease => handle_frame!(frame, FReleaseHandler),
                    _ => {
                        todo!("Handle other frame types here");
                    }
                },
                Err(e) => {
                    log::error!("Error processing frame: {e}");
                    return Err(e);
                }
            }
        }
        Ok(())
    }
}

impl Drop for PesitServerHandler {
    fn drop(&mut self) {
        log::info!(
            "Dropping PesitServerHandler, last state was = {:?}",
            self.state
        );
    }
}
