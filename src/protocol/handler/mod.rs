use std::{borrow::Cow, future::Future};

use crate::{
    connection::PesitFramedStream, error::PesitError, protocol::frame::Frame, state::State,
};

pub(crate) mod fpdu;
pub(crate) mod prelude;

/// Trait for handling PESIT frames.
/// This trait is implemented to handle Frames for a specific State for Client or Server.
pub(crate) trait FrameHandler<'r, S: State> {
    type RawPayload;
    /// The type of the payload extracted from the frame.
    /// It usually contains a tuple containing all the protocol information that can be extracted.
    type PiPayload;

    /// Handles the incoming frame, implements the logic behind FPDU and updates the connection state.
    fn handle(
        conn: &mut PesitFramedStream,
        frame: Frame<Self::RawPayload>,
        state: &mut S,
    ) -> impl Future<Output = Result<(), PesitError>> + Send;

    /// Extracts the payload from the incoming frame.
    fn extract_payload(
        frame: &'r Frame<Self::RawPayload>,
    ) -> Result<(Self::RawPayload, Self::PiPayload), PesitError>;
}

pub fn convert_frame_owned<'r>(f: Frame<Vec<u8>>) -> Frame<Cow<'r, [u8]>> {
    Frame {
        header: f.header,
        payload: Cow::Owned(f.payload),
        len: f.len,
    }
}
