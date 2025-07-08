use super::prelude::*;

pub(crate) struct FReleaseHandler {}

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
