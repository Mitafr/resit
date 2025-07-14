use std::borrow::Cow;

use super::prelude::*;

pub(crate) struct FConnectHandler {}

impl<'r> FrameHandler<'r, ServerState> for FConnectHandler {
    type RawPayload = Cow<'r, [u8]>;

    type PiPayload = (
        Pi1,
        Pi3,
        Pi4,
        Option<Pi5>,
        Pi6,
        Pi7,
        //Pi22,
        //Pi23,
        Option<Pi99>,
    );

    async fn handle(
        conn: &mut PesitFramedStream,
        frame: Frame<Self::RawPayload>,
        state: &mut ServerState,
    ) -> Result<(), PesitError> {
        log::debug!("Handling FConnect frame");
        if *state == ServerState::Connected {
            log::warn!("Received FConnect frame while already connected.");
        }
        let (_, payload) = Self::extract_payload(&frame).unwrap();
        log::info!("{payload:?}");
        conn.send(Frame {
            header: FrameHeader {
                kind: FrameType::FAConnect,
                msg_type: 0x21,
                dest_id: 0x0,
                oct6: rand::random::<u8>(),
                length: 0,
            },
            payload: vec![0u8],
            len: 0,
        })
        .await?;
        *state = ServerState::Connected;
        Ok(())
    }

    fn extract_payload(
        frame: &'r Frame<Self::RawPayload>,
    ) -> Result<(Self::RawPayload, Self::PiPayload), PesitError> {
        let raw_payload = &frame.payload;
        let (raw_payload, pi1) = parse_pi::<Pi1>(raw_payload).unwrap_or_default();
        let (raw_payload, pi3) = parse_pi::<Pi3>(raw_payload)?;
        let (raw_payload, pi4) = parse_pi::<Pi4>(raw_payload)?;
        let (raw_payload, pi5) = parse_pi::<Pi5>(raw_payload)?;
        let (raw_payload, pi6) = parse_pi::<Pi6>(raw_payload)?;
        let (_raw_payload, pi7) = parse_pi::<Pi7>(raw_payload)?;
        Ok((
            Cow::Borrowed(raw_payload),
            (pi1, pi3, pi4, Some(pi5), pi6, pi7, None),
        ))
    }
}
