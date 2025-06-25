use std::{future::Future, io::Error, sync::Arc};

use futures::SinkExt;
use tokio::{
    net::{TcpListener, TcpStream},
    sync::Mutex,
};
use tokio_stream::StreamExt;

use crate::{
    connection::PesitFramedStream,
    error::PesitError,
    protocol::{
        codec::PesitCodec,
        frame::{handler::FConnectHandler, types::FrameType, Frame},
    },
    state::{ServerState, State},
};

pub trait FrameHandler<S: State> {
    fn handle(
        &self,
        conn: &mut PesitFramedStream,
        frame: Frame,
    ) -> impl Future<Output = Result<S, PesitError>> + Send;
}

#[derive(Debug)]
pub struct PesitServer {
    port: u16,
    listener: TcpListener,
    state: Arc<Mutex<ServerState>>,
}

impl PesitServer {
    pub async fn new(port: u16) -> Result<Self, Error> {
        let addr = format!("127.0.0.1:{}", port);

        let listener = match TcpListener::bind(&addr).await {
            Ok(tcp_listener) => {
                log::info!("TCP listener started on port {port}");
                tcp_listener
            }
            Err(e) => panic!("Could not bind the TCP listener to {}. Err: {}", &addr, e),
        };

        Ok(Self {
            port,
            listener,
            state: Arc::new(Mutex::new(ServerState::default())),
        })
    }

    pub async fn run(&mut self) -> Result<(), Error> {
        loop {
            let sock = match self.accept_conn().await {
                Ok(stream) => stream,
                Err(e) => {
                    panic!("Error accepting connection on port {}: {}", self.port, e);
                }
            };
            log::info!("Accepted connection from {}", sock.peer_addr()?);

            let resp_command_frame = PesitFramedStream::with_capacity(sock, PesitCodec, 8 * 1024);
            let s = Arc::clone(&self.state);
            tokio::spawn(async move {
                let mut handler = PesitServerHandler::new(resp_command_frame);
                if let Err(e) = handler.handle(s).await {
                    log::error!("Failed to handle command: {}", e);
                }
            });
        }
    }

    async fn accept_conn(&mut self) -> Result<TcpStream, Error> {
        match self.listener.accept().await {
            Ok((sock, _)) => Ok(sock),
            Err(e) => Err(e),
        }
    }
}

struct PesitServerHandler {
    conn: PesitFramedStream,
}

impl PesitServerHandler {
    pub fn new(conn: PesitFramedStream) -> Self {
        Self { conn }
    }

    pub async fn handle(&mut self, mut state: Arc<Mutex<ServerState>>) -> Result<(), PesitError> {
        while let Some(frame) = self.conn.next().await {
            match frame {
                Ok(frame) => match frame.header.kind {
                    FrameType::FConnect => {
                        let mut lock = state.lock().await;
                        if *lock == ServerState::Connected {
                            log::warn!("Received FConnect frame while already connected.");
                        }
                        *lock = FConnectHandler {}.handle(&mut self.conn, frame).await?;
                    }
                    FrameType::FRelease => {
                        log::info!("Received FRelease frame, disconnecting.");
                        self.conn.close().await?;
                        *state.lock().await = ServerState::Disconnected;
                    }
                    _ => {
                        todo!("Handle other frame types here");
                    }
                },
                Err(e) => {
                    log::error!("Error processing frame: {}", e);
                    return Err(e);
                }
            }
        }
        Ok(())
    }
}
