use crate::{error::PesitError, protocol::frame::types::FrameType};

pub(crate) mod handler;
pub(crate) mod types;

#[derive(Debug, Default)]
pub(crate) struct Frame {
    pub header: FrameHeader,
    pub payload: Vec<u8>,
    pub len: usize,
}

impl Frame {
    pub fn new() -> Self {
        Self {
            header: FrameHeader {
                kind: FrameType::Unknown,
                length: 0,
                msg_type: 0,
                dest_id: 0,
                oct6: 0,
            },
            payload: Vec::new(),
            len: 10,
        }
    }

    pub fn from_bytes(input: &[u8]) -> Result<Self, PesitError> {
        let header = FrameHeader::from_bytes(input)?;
        let payload_start = 3;
        let payload = input[payload_start..].to_vec();

        Ok(Self {
            header,
            payload,
            len: input.len(),
        })
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub(crate) struct FrameHeader {
    pub kind: FrameType,
    pub length: u16,
    pub msg_type: u8,
    pub dest_id: u8,
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
