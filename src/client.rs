use crate::connection::PesitFramedStream;
use crate::protocol::frame::types::FrameType;
use crate::protocol::frame::FrameHeader;
use crate::{connection::connect, error::PesitError, protocol::frame::Frame, state::ClientState};
use futures::SinkExt;
use tokio_stream::StreamExt;

pub struct PesitClient {
    pub stream: PesitFramedStream,
    pub state: ClientState,
}

impl PesitClient {
    pub async fn connect(addr: &str) -> Result<Self, PesitError> {
        let stream = connect(addr).await?;
        log::info!("Connected to server at {}", addr);
        Ok(Self {
            stream,
            state: ClientState::Idle,
        })
    }

    pub async fn init_connection(&mut self) -> Result<(), PesitError> {
        self.send_frame(Frame {
            header: FrameHeader {
                kind: FrameType::FConnect,
                msg_type: 0x20,
                dest_id: 0x0,
                oct6: rand::random::<u8>(),
                length: 0,
            },
            payload: vec![],
            len: 0,
        })
        .await?;
        if let Some(frame) = self.stream.next().await {
            match frame {
                Ok(frame) => {
                    if frame.header.kind == FrameType::FAConnect {
                        self.state = ClientState::Connected;
                        log::info!("Connection established with server.");
                        return Ok(());
                    }
                }
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }

    pub async fn send_frame(&mut self, frame: Frame) -> Result<(), PesitError> {
        log::debug!("Sending frame: {:?}", frame);
        self.stream.send(frame).await.map_err(Into::into)
    }

    pub async fn receive_frame(&mut self) -> Result<Frame, PesitError> {
        self.stream.next().await.ok_or(PesitError::Protocol)?
    }

    pub async fn disconnect(&mut self) -> Result<(), PesitError> {
        log::info!("Disconnecting from server.");
        self.send_frame(Frame {
            header: FrameHeader {
                kind: FrameType::FRelease,
                msg_type: 0x23,
                dest_id: 0x0,
                oct6: rand::random::<u8>(),
                length: 0,
            },
            payload: vec![],
            len: 0,
        })
        .await?;
        self.state = ClientState::Disconnected;
        Ok(())
    }
}
