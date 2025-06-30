use futures::SinkExt;
use nom::IResult;

use crate::{
    connection::PesitFramedStream,
    error::PesitError,
    protocol::{
        frame::{types::FrameType, Frame, FrameHeader},
        pi::{Pi, Pi1, Pi3, Pi4, Pi5, Pi6, Pi7, Pi99},
    },
    server::FrameHandler,
    state::ServerState,
};

fn parse_pi<P: Pi>(input: &[u8]) -> IResult<&[u8], P> {
    P::parse(input)
}

pub(crate) struct FConnectHandler {}
pub(crate) struct FReleaseHandler {}

impl FrameHandler<ServerState> for FConnectHandler {
    type Payload = Result<
        (
            Pi1,
            Pi3,
            Pi4,
            Option<Pi5>,
            Pi6,
            Pi7,
            //Pi22,
            //Pi23,
            Option<Pi99>,
        ),
        PesitError,
    >;

    async fn handle(
        &self,
        conn: &mut PesitFramedStream,
        frame: Frame,
        state: &mut ServerState,
    ) -> Result<(), PesitError> {
        log::debug!("Handling FConnect frame");
        if *state == ServerState::Connected {
            log::warn!("Received FConnect frame while already connected.");
        }
        conn.send(Frame {
            header: FrameHeader {
                kind: FrameType::FAConnect,
                msg_type: 0x21,
                dest_id: 0x0,
                oct6: rand::random::<u8>(),
                length: 0,
            },
            payload: vec![],
            len: 0,
        })
        .await?;
        let payload = self.extract_payload(&frame)?;
        log::info!("{payload:?}");
        *state = ServerState::Connected;
        Ok(())
    }

    fn extract_payload(&self, frame: &Frame) -> Self::Payload {
        let raw_payload = &frame.payload;
        let (raw_payload, pi1) = parse_pi::<Pi1>(raw_payload).unwrap_or_default();
        let (raw_payload, pi3) = parse_pi::<Pi3>(raw_payload).unwrap_or_default();
        let (raw_payload, pi4) = parse_pi::<Pi4>(raw_payload).unwrap_or_default();
        let (raw_payload, pi5) = parse_pi::<Pi5>(raw_payload).unwrap_or_default();
        let (raw_payload, pi6) = parse_pi::<Pi6>(raw_payload).unwrap_or_default();
        let (_raw_payload, pi7) = parse_pi::<Pi7>(raw_payload).unwrap_or_default();
        Ok((pi1, pi3, pi4, Some(pi5), pi6, pi7, None))
    }
}

impl FrameHandler<ServerState> for FReleaseHandler {
    type Payload = ();
    async fn handle(
        &self,
        conn: &mut PesitFramedStream,
        _frame: Frame,
        state: &mut ServerState,
    ) -> Result<(), PesitError> {
        log::debug!("Handling FRelease frame");
        conn.send(
            Frame::builder()
                .header(
                    FrameHeader::builder()
                        .kind(FrameType::FRelconf)
                        .msg_type(0x24)
                        .dest_id(0x0)
                        .length(0)
                        .build(),
                )
                .payload(vec![])
                .len(0)
                .build(),
        )
        .await?;
        conn.close().await?;
        *state = ServerState::Disconnected;
        log::info!("Client Disconnected.");

        Ok(())
    }

    fn extract_payload(&self, _frame: &Frame) -> Self::Payload {
        ()
    }
}
