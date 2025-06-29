use nom::IResult;

use crate::protocol::pi::Pi;

#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) enum ExchangeType {
    Send,
    Receive,
    #[default]
    Ack,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Pi11([u8; 2]);

impl Pi11 {
    pub fn exchange_type(&self) -> ExchangeType {
        if self.0 == 0xFFFFu16.to_be_bytes() {
            ExchangeType::Send
        } else if self.0 == 0xFFFEu16.to_be_bytes() {
            ExchangeType::Receive
        } else {
            ExchangeType::Ack
        }
    }
}

impl Pi for Pi11 {
    fn parse(data: &[u8]) -> IResult<&[u8], Self>
    where
        Self: Sized,
    {
        if data.len() < 2 {
            return Err(nom::Err::Incomplete(nom::Needed::new(2)));
        }
        let mut bytes = [0u8; 2];
        bytes.copy_from_slice(&data[..2]);
        Ok((&data[2..], Pi11(bytes)))
    }
}
