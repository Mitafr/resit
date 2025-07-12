use bon::Builder;

use crate::{error::PesitError, protocol::frame::types::FrameType};

pub(crate) mod types;

#[derive(Debug, Default, Clone, PartialEq, Builder)]
pub(crate) struct Frame<P> {
    pub header: FrameHeader,
    pub payload: P,
    pub len: usize,
}

impl TryFrom<Vec<u8>> for Frame<Vec<u8>> {
    type Error = PesitError;
    fn try_from(value: Vec<u8>) -> Result<Frame<Vec<u8>>, Self::Error> {
        let header = FrameHeader::from_bytes(&value)?;
        let payload_start = 3;
        let payload = value[payload_start..].to_vec();

        Ok(Self {
            header,
            payload,
            len: value.len(),
        })
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Builder)]
pub(crate) struct FrameHeader {
    pub kind: FrameType,
    pub length: u16,
    #[builder(default)]
    pub msg_type: u8,
    #[builder(default)]
    pub dest_id: u8,
    #[builder(default)]
    pub oct6: u8,
}

impl FrameHeader {
    pub fn from_bytes(input: &[u8]) -> Result<Self, PesitError> {
        if input.len() < 6 {
            return Err(PesitError::InvalidFrame);
        }

        let kind = <[u8; 4]>::try_from(&input[3..=6]).unwrap_or_default();

        Ok(Self {
            kind: FrameType::from(kind),
            msg_type: input[3],
            dest_id: input[4],
            oct6: input[5],
            length: u16::from_be_bytes([input[1], input[2]]),
        })
    }
}
