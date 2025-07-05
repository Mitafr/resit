use futures::SinkExt;
use nom::IResult;

use crate::{
    connection::PesitFramedStream,
    error::PesitError,
    protocol::{
        frame::{types::FrameType, Frame, FrameHeader},
        pi::prelude::*,
    },
    server::FrameHandler,
    state::ServerState,
};

fn parse_pi<P: Pi>(input: &[u8]) -> IResult<&[u8], P> {
    P::parse(input)
}

pub(crate) struct FCreateHandler {}
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

impl FrameHandler<ServerState> for FCreateHandler {
    type Payload = Result<
        (
            Option<Pi3>,
            Option<Pi4>,
            Pi11,
            Pi12,
            Pi13,
            Pi15,
            Pi16,
            Pi17,
            Pi25,
            Pi31,
            Pi32,
            Pi33,
            Pi36,
            Pi37,
            Pi41,
            Pi42,
            Pi51,
            Pi52,
            Pi61,
            Pi62,
            Pi99,
        ),
        PesitError,
    >;
    async fn handle(
        &self,
        conn: &mut PesitFramedStream,
        frame: Frame,
        state: &mut ServerState,
    ) -> Result<(), PesitError> {
        log::debug!("Handling FCreate frame");
        let payload = self.extract_payload(&frame)?;
        log::info!(
            "FCreate payload: pi3={:?}, pi4={:?}, pi11={:?}, pi12={:?}, pi13={:?}, pi15={:?}, pi16={:?}, pi17={:?}, pi25={:?}, pi31={:?}, pi32={:?}, pi33={:?}, pi36={:?}, pi37={:?}, pi41={:?}, pi42={:?}, pi51={:?}, pi52={:?}, pi61={:?}, pi62={:?}, pi99={:?}",
            payload.0,
            payload.1,
            payload.2,
            payload.3,
            payload.4,
            payload.5,
            payload.6,
            payload.7,
            payload.8,
            payload.9,
            payload.10,
            payload.11,
            payload.12,
            payload.13,
            payload.14,
            payload.15,
            payload.16,
            payload.17,
            payload.18,
            payload.19,
            payload.20,
        );
        *state = ServerState::Connected;
        Ok(())
    }

    fn extract_payload(&self, frame: &Frame) -> Self::Payload {
        let raw_payload = &frame.payload;
        let (raw_payload, pi3) = parse_pi::<Pi3>(raw_payload).unwrap_or_default();
        let (raw_payload, pi4) = parse_pi::<Pi4>(raw_payload).unwrap_or_default();
        let (raw_payload, pi11) = parse_pi::<Pi11>(raw_payload).unwrap_or_default();
        let (raw_payload, pi12) = parse_pi::<Pi12>(raw_payload).unwrap_or_default();
        let (raw_payload, pi13) = parse_pi::<Pi13>(raw_payload).unwrap_or_default();
        let (raw_payload, pi15) = parse_pi::<Pi15>(raw_payload).unwrap_or_default();
        let (raw_payload, pi16) = parse_pi::<Pi16>(raw_payload).unwrap_or_default();
        let (raw_payload, pi17) = parse_pi::<Pi17>(raw_payload).unwrap_or_default();
        let (raw_payload, pi25) = parse_pi::<Pi25>(raw_payload).unwrap_or_default();
        let (raw_payload, pi31) = parse_pi::<Pi31>(raw_payload).unwrap_or_default();
        let (raw_payload, pi32) = parse_pi::<Pi32>(raw_payload).unwrap_or_default();
        let (raw_payload, pi33) = parse_pi::<Pi33>(raw_payload).unwrap_or_default();
        let (raw_payload, pi36) = parse_pi::<Pi36>(raw_payload).unwrap_or_default();
        let (raw_payload, pi37) = parse_pi::<Pi37>(raw_payload).unwrap_or_default();
        let (raw_payload, pi41) = parse_pi::<Pi41>(raw_payload).unwrap_or_default();
        let (raw_payload, pi42) = parse_pi::<Pi42>(raw_payload).unwrap_or_default();
        let (raw_payload, pi51) = parse_pi::<Pi51>(raw_payload).unwrap_or_default();
        let (raw_payload, pi52) = parse_pi::<Pi52>(raw_payload).unwrap_or_default();
        let (raw_payload, pi61) = parse_pi::<Pi61>(raw_payload).unwrap_or_default();
        let (raw_payload, pi62) = parse_pi::<Pi62>(raw_payload).unwrap_or_default();
        let (_raw_payload, pi99) = parse_pi::<Pi99>(raw_payload).unwrap_or_default();
        Ok((
            Some(pi3),
            Some(pi4),
            pi11,
            pi12,
            pi13,
            pi15,
            pi16,
            pi17,
            pi25,
            pi31,
            pi32,
            pi33,
            pi36,
            pi37,
            pi41,
            pi42,
            pi51,
            pi52,
            pi61,
            pi62,
            pi99,
        ))
    }
}
