use futures::SinkExt;

use crate::{
    connection::PesitFramedStream,
    error::PesitError,
    protocol::frame::{types::FrameType, Frame, FrameHeader},
    server::FrameHandler,
    state::ServerState,
};

pub struct FConnectHandler {}

impl FrameHandler<ServerState> for FConnectHandler {
    async fn handle(
        &self,
        conn: &mut PesitFramedStream,
        _frame: Frame,
    ) -> Result<ServerState, PesitError> {
        log::debug!("Handling FConnect frame");
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
        Ok(ServerState::Connected)
    }
}
