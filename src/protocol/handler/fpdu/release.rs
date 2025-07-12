use std::borrow::Cow;

use super::prelude::*;

pub(crate) struct FReleaseHandler {}

impl<'r> FrameHandler<'r, ServerState> for FReleaseHandler {
    type RawPayload = Cow<'r, [u8]>;

    type PiPayload = ();

    async fn handle(
        conn: &mut PesitFramedStream,
        _frame: Frame<Self::RawPayload>,
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
                .payload(vec![0u8])
                .len(0)
                .build(),
        )
        .await?;
        conn.close().await?;
        *state = ServerState::Disconnected;
        log::info!("Client Disconnected.");

        Ok(())
    }

    fn extract_payload(
        frame: &'r Frame<Self::RawPayload>,
    ) -> Result<(Self::RawPayload, Self::PiPayload), PesitError> {
        let raw_payload = &frame.payload;
        Ok((Cow::Borrowed(raw_payload), ()))
    }
}
