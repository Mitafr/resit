use super::prelude::*;

pub(crate) struct FCreateHandler {}
pub(crate) struct FAckCreateHandler {}

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
        *state = ServerState::FileSelection;
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
                Pi2 {
                    error_type: 0,
                    reason_code: 0,
                }
                .to_bytes(),
            )
            .len(0)
            .build();
        conn.send(ack_frame).await?;
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

impl FrameHandler<ClientState> for FAckCreateHandler {
    type Payload = (Pi2, Option<Pi13>, Pi25, Option<Pi99>);
    async fn handle(
        &self,
        _conn: &mut PesitFramedStream,
        frame: Frame,
        state: &mut ClientState,
    ) -> Result<(), PesitError> {
        let _ = self.extract_payload(&frame);
        log::info!("Handling FAckCreate frame: {frame:?}");
        *state = ClientState::FileSelection;
        Ok(())
    }

    fn extract_payload(&self, frame: &Frame) -> Self::Payload {
        let raw_payload = &frame.payload;
        let (raw_payload, pi2) = parse_pi::<Pi2>(raw_payload).unwrap_or_default();
        let (raw_payload, pi13) = parse_pi::<Pi13>(raw_payload).unwrap_or_default();
        let (raw_payload, pi25) = parse_pi::<Pi25>(raw_payload).unwrap_or_default();
        let (_raw_payload, pi99) = parse_pi::<Pi99>(raw_payload).unwrap_or_default();
        (pi2, Some(pi13), pi25, Some(pi99))
    }
}
