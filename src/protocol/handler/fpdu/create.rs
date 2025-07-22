use std::borrow::Cow;

use super::prelude::*;

pub(crate) struct FCreateHandler {}
pub(crate) struct FAckCreateHandler {}

#[derive(Debug)]
pub struct FCreatePayload {
    pi3: Option<Pi3>,
    pi4: Option<Pi4>,
    pi11: Pi11,
    pi12: Pi12,
    pi13: Pi13,
    pi15: Pi15,
    pi16: Pi16,
    pi17: Pi17,
    pi25: Pi25,
    pi31: Pi31,
    pi32: Pi32,
    pi33: Pi33,
    pi37: Pi37,
    pi38: Pi38,
    pi41: Pi41,
    pi42: Pi42,
    pi51: Pi51,
    pi52: Pi52,
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
        log::debug!("Asked to create this Pi12 : {:?}", payload.pi12);
        let pi2 = Pi2 {
            error_type: 0,
            reason_code: 0,
        }
        .as_bytes();
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
            .payload(pi2)
            .len(0)
            .build();
        conn.send(ack_frame).await?;
        Ok(())
    }

    fn extract_payload(
        frame: &'r Frame<Self::RawPayload>,
    ) -> Result<(Self::RawPayload, Self::PiPayload), PesitError> {
        let raw_payload = &frame.payload;
        log::debug!("{raw_payload:#?}");
        let (raw_payload, pi3) = parse_pi::<Pi3>(raw_payload).unwrap();
        log::debug!("{raw_payload:#?}");
        let (raw_payload, pi4) = parse_pi::<Pi4>(raw_payload).unwrap();
        let (raw_payload, pi11) = parse_pi::<Pi11>(raw_payload).unwrap();
        let (raw_payload, pi12) = parse_pi::<Pi12>(raw_payload).unwrap();
        let (raw_payload, pi13) = parse_pi::<Pi13>(raw_payload).unwrap();
        let (raw_payload, pi15) = parse_pi::<Pi15>(raw_payload).unwrap();
        let (raw_payload, pi16) = parse_pi::<Pi16>(raw_payload).unwrap();
        let (raw_payload, pi17) = parse_pi::<Pi17>(raw_payload).unwrap();
        let (raw_payload, pi25) = parse_pi::<Pi25>(raw_payload).unwrap();
        let (raw_payload, pi31) = parse_pi::<Pi31>(raw_payload).unwrap();
        let (raw_payload, pi32) = parse_pi::<Pi32>(raw_payload).unwrap();
        let (raw_payload, pi33) = parse_pi::<Pi33>(raw_payload).unwrap();
        let (raw_payload, pi37) = parse_pi::<Pi37>(raw_payload).unwrap();
        let (raw_payload, pi38) = parse_pi::<Pi38>(raw_payload).unwrap();
        let (raw_payload, pi41) = parse_pi::<Pi41>(raw_payload).unwrap();
        let (raw_payload, pi42) = parse_pi::<Pi42>(raw_payload).unwrap();
        let (raw_payload, pi51) = parse_pi::<Pi51>(raw_payload).unwrap();
        let (raw_payload, pi52) = parse_pi::<Pi52>(raw_payload).unwrap();
        let (raw_payload, pi61) = parse_pi::<Pi61>(raw_payload).unwrap();
        let (raw_payload, pi62) = parse_pi::<Pi62>(raw_payload).unwrap();
        let (_raw_payload, pi99) = parse_pi::<Pi99>(raw_payload).unwrap();
        Ok((
            Cow::Borrowed(raw_payload),
            FCreatePayload {
                pi3: Some(pi3),
                pi4: Some(pi4),
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
                pi37,
                pi38,
                pi41,
                pi42,
                pi51,
                pi52,
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
        let (_raw_payload, pi99) = parse_pi::<Pi99>(raw_payload)?;
        Ok((
            Cow::Borrowed(raw_payload),
            (pi2, Some(pi13), pi25, Some(pi99)),
        ))
    }
}
