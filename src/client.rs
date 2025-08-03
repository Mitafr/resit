use crate::connection::PesitFramedStream;
use crate::protocol::frame::types::FrameType;
use crate::protocol::frame::FrameHeader;
use crate::protocol::handler::{convert_frame_owned, prelude::*, FrameHandler};
use crate::protocol::pgi::file_desc::{FileDescriptor, FileDescriptorPis};
use crate::protocol::pgi::hist_attr::HistoricAttribute;
use crate::protocol::pgi::log_attr::LogicalAttribute;
use crate::protocol::pgi::phys_attr::PhysicalAttribute;
use crate::protocol::pi::prelude::*;
use crate::{connection::connect, error::PesitError, protocol::frame::Frame, state::ClientState};
use futures::SinkExt;
use tokio_stream::StreamExt;

#[derive(Debug)]
pub struct PesitClient {
    pub stream: PesitFramedStream,
    pub state: ClientState,
    id: u32,
}

impl PesitClient {
    pub async fn connect(addr: &str) -> Result<Self, PesitError> {
        let stream = connect(addr).await?;
        log::info!("Connected to server at {addr}");
        Ok(Self {
            stream,
            state: ClientState::Idle,
            id: rand::random(),
        })
    }

    pub async fn init_connection(&mut self) -> Result<(), PesitError> {
        let frame = Frame::builder()
            .header(
                FrameHeader::builder()
                    .kind(FrameType::FConnect)
                    .msg_type(0x20)
                    .dest_id(0x0)
                    .oct6(rand::random::<u8>())
                    .length(0)
                    .build(),
            )
            .payload(
                vec![
                    Pi1(false).as_bytes(),
                    Pi3::default().as_bytes(),
                    Pi4::default().as_bytes(),
                    Pi5::default().as_bytes(),
                    Pi6::default().as_bytes(),
                    Pi7::default().as_bytes(),
                    Pi22::default().as_bytes(),
                    Pi23::default().as_bytes(),
                    Pi99::default().as_bytes(),
                ]
                .into_iter()
                .flatten()
                .collect(),
            )
            .len(0)
            .build();
        self.send_frame(frame).await?;
        if let Ok(frame) = self.receive_frame().await {
            if frame.header.kind == FrameType::FAConnect {
                self.state = ClientState::Connected;
                log::info!("Connection established with server.");
                return Ok(());
            }
        }
        Ok(())
    }

    pub async fn send_frame(&mut self, frame: Frame<Vec<u8>>) -> Result<(), PesitError> {
        log::debug!("Sending frame: {frame:?}");
        self.stream.send(frame).await?;
        Ok(())
    }

    pub async fn receive_frame(&mut self) -> Result<Frame<Vec<u8>>, PesitError> {
        let frame = self.stream.next().await.ok_or(PesitError::Protocol)?;
        log::debug!("Received frame: {frame:?}");
        frame
    }

    pub async fn disconnect(&mut self) -> Result<(), PesitError> {
        log::info!("Disconnecting from server.");
        let frame = Frame::builder()
            .header(
                FrameHeader::builder()
                    .kind(FrameType::FRelease)
                    .msg_type(0x23)
                    .dest_id(0x0)
                    .oct6(rand::random::<u8>())
                    .length(0)
                    .build(),
            )
            .payload(vec![])
            .len(10)
            .build();
        self.send_frame(frame).await?;
        if let Ok(confirm_frame) = self.receive_frame().await {
            if confirm_frame.header.kind == FrameType::FRelconf {
                log::info!("Server confirmed disconnection.");
            } else {
                log::warn!("Received unexpected frame during disconnection: {confirm_frame:?}");
            }
        }
        self.state = ClientState::Disconnected;
        Ok(())
    }

    pub(crate) async fn create(&mut self) -> Result<(), PesitError> {
        let file_desc = FileDescriptor::builder()
            .pis(
                FileDescriptorPis::builder()
                    .pi12(
                        Pi12::builder()
                            .identifier(crate::protocol::pi::pi12::IdentifierType::Standard)
                            .file_reference(*b"A24070124071")
                            .build(),
                    )
                    .build(),
            )
            .build();
        let log_attr = LogicalAttribute::builder().build();
        let phys_attr = PhysicalAttribute::builder().build();
        let hist_attr = HistoricAttribute::builder().build();
        let frame = Frame::builder()
            .header(
                FrameHeader::builder()
                    .kind(FrameType::FCreate)
                    .msg_type(0x22)
                    .dest_id(0x0)
                    .oct6(0)
                    .length(0)
                    .build(),
            )
            .payload(
                vec![
                    file_desc.as_bytes(),
                    Pi13::default().as_bytes(),
                    Pi15::default().as_bytes(),
                    Pi16::default().as_bytes(),
                    Pi17::default().as_bytes(),
                    Pi25::default().as_bytes(),
                    log_attr.as_bytes(),
                    phys_attr.as_bytes(),
                    hist_attr.as_bytes(),
                    Pi61::default().as_bytes(),
                    Pi62::default().as_bytes(),
                    Pi99::from_str("test123456").as_bytes(),
                ]
                .into_iter()
                .flatten()
                .collect(),
            )
            .len(0)
            .build();
        self.send_frame(frame).await?;
        if let Ok(confirm_frame) = self.receive_frame().await {
            if confirm_frame.header.kind == FrameType::FAckCreate {
                FAckCreateHandler::handle(
                    &mut self.stream,
                    convert_frame_owned(confirm_frame),
                    &mut self.state,
                )
                .await?;
            } else {
                log::warn!("Received unexpected frame during creation: {confirm_frame:?}");
            }
        }
        Ok(())
    }
}
