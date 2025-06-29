use crate::protocol::frame::{types::FrameType, FrameHeader};

use super::frame::Frame;

#[derive(Debug, Default)]
pub(crate) struct Parser {}

impl Parser {
    pub fn parse(self, input: &[u8]) -> Frame {
        let header = FrameHeader {
            kind: FrameType::from_header(<&[u8; 4]>::try_from(&input[2..6]).unwrap()),
            ..Default::default()
        };
        log::debug!("Parsed header: {:?}", header);
        // parse_pi::<Pi71>(input).expect("Parse error");
        Frame {
            header,
            payload: input[6..].to_vec(),
            len: input.len(),
        }
    }
}
