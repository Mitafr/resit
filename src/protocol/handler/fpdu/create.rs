use std::borrow::Cow;

use crate::protocol::pgi::{
    file_desc::FileDescriptor, hist_attr::HistoricAttribute, log_attr::LogicalAttribute,
    phys_attr::PhysicalAttribute, Pgi,
};

use super::prelude::*;

pub(crate) struct FCreateHandler {}
pub(crate) struct FAckCreateHandler {}

#[derive(Debug)]
pub struct FCreatePayload {
    pgi9: FileDescriptor,
    pi13: Pi13,
    pi15: Pi15,
    pi16: Pi16,
    pi17: Pi17,
    pi25: Pi25,
    pgi30: LogicalAttribute,
    pgi40: PhysicalAttribute,
    pgi50: HistoricAttribute,
    pi61: Pi61,
    pi62: Pi62,
    pi99: Pi99,
}

impl<'r> FrameHandler<'r, ServerState> for FCreateHandler {
    type RawPayload = Cow<'r, [u8]>;

    type PiPayload = FCreatePayload;

    async fn handle(
        conn: &mut PesitFramedStream,
        frame: Frame<Self::RawPayload>,
        state: &mut ServerState,
    ) -> Result<(), PesitError> {
        log::debug!("Handling FCreate frame");
        let (_raw, payload) = Self::extract_payload(&frame).unwrap();
        log::info!("{payload:?}");
        *state = ServerState::FileSelection;
        log::debug!("Asked to create this Pi12 : {}", payload.pgi9.pis.pi12);
        let ack_frame = Frame::builder()
            .header(
                FrameHeader::builder()
                    .kind(FrameType::FAckCreate)
                    .msg_type(0x00)
                    .dest_id(0x0)
                    .oct6(rand::random::<u8>())
                    .length(0)
                    .build(),
            )
            .payload(
                vec![Pi2 {
                    error_type: 0,
                    reason_code: 0,
                }
                .as_bytes()]
                .into_iter()
                .flatten()
                .collect(),
            )
            .len(0)
            .build();
        conn.send(ack_frame).await?;
        Ok(())
    }

    fn extract_payload(
        frame: &'r Frame<Self::RawPayload>,
    ) -> Result<(Self::RawPayload, Self::PiPayload), PesitError> {
        let raw_payload = &frame.payload;
        log::debug!("{raw_payload:?}");
        let (raw_payload, pgi9) = FileDescriptor::parse(&raw_payload).unwrap();
        let (raw_payload, pi13) = parse_pi::<Pi13>(raw_payload).unwrap();
        let (raw_payload, pi15) = parse_pi::<Pi15>(raw_payload).unwrap();
        let (raw_payload, pi16) = parse_pi::<Pi16>(raw_payload).unwrap();
        let (raw_payload, pi17) = parse_pi::<Pi17>(raw_payload).unwrap();
        let (raw_payload, pi25) = parse_pi::<Pi25>(raw_payload).unwrap();
        let (raw_payload, pgi30) = LogicalAttribute::parse(raw_payload).unwrap();
        let (raw_payload, pgi40) = PhysicalAttribute::parse(raw_payload).unwrap();
        let (raw_payload, pgi50) = HistoricAttribute::parse(raw_payload).unwrap();
        let (raw_payload, pi61) = parse_pi::<Pi61>(raw_payload).unwrap();
        let (raw_payload, pi62) = parse_pi::<Pi62>(raw_payload).unwrap();
        let (_raw_payload, pi99) = parse_pi::<Pi99>(raw_payload).unwrap();
        Ok((
            Cow::Borrowed(raw_payload),
            FCreatePayload {
                pgi9,
                pi13,
                pi15,
                pi16,
                pi17,
                pi25,
                pgi30,
                pgi40,
                pgi50,
                pi61,
                pi62,
                pi99,
            },
        ))
    }
}

impl<'r> FrameHandler<'r, ClientState> for FAckCreateHandler {
    type RawPayload = Cow<'r, [u8]>;

    type PiPayload = (Pi2, Option<Pi13>, Pi25, Option<Pi99>);

    async fn handle(
        _conn: &mut PesitFramedStream,
        frame: Frame<Self::RawPayload>,
        state: &mut ClientState,
    ) -> Result<(), PesitError> {
        let _ = Self::extract_payload(&frame);
        log::info!("Handling FAckCreate frame: {frame:?}");
        *state = ClientState::FileSelection;
        Ok(())
    }

    fn extract_payload(
        frame: &'r Frame<Self::RawPayload>,
    ) -> Result<(Self::RawPayload, Self::PiPayload), PesitError> {
        let raw_payload = &frame.payload;
        let (raw_payload, pi2) = parse_pi::<Pi2>(raw_payload)?;
        let (raw_payload, pi13) = parse_pi::<Pi13>(raw_payload)?;
        let (raw_payload, pi25) = parse_pi::<Pi25>(raw_payload)?;
        let (raw_payload, pi99) = parse_pi::<Pi99>(raw_payload)?;
        Ok((
            Cow::Borrowed(raw_payload),
            (pi2, Some(pi13), pi25, Some(pi99)),
        ))
    }
}
