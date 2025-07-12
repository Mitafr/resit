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

    fn as_bytes(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    fn code(&self) -> u8 {
        11
    }

    fn len(&self) -> usize {
        2
    }

    fn ptype(&self) -> super::PiType {
        super::PiType::N
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_send() {
        let data = 0xFFFFu16.to_be_bytes();
        let input = &data[..];
        let (rem, pi) = Pi11::parse(input).unwrap();
        assert_eq!(rem.len(), 0);
        assert_eq!(pi.0, data);
        assert_eq!(pi.exchange_type(), ExchangeType::Send);
    }

    #[test]
    fn test_parse_receive() {
        let data = 0xFFFEu16.to_be_bytes();
        let input = &data[..];
        let (rem, pi) = Pi11::parse(input).unwrap();
        assert_eq!(rem.len(), 0);
        assert_eq!(pi.0, data);
        assert_eq!(pi.exchange_type(), ExchangeType::Receive);
    }

    #[test]
    fn test_parse_ack() {
        let data = 0x1234u16.to_be_bytes();
        let input = &data[..];
        let (rem, pi) = Pi11::parse(input).unwrap();
        assert_eq!(rem.len(), 0);
        assert_eq!(pi.0, data);
        assert_eq!(pi.exchange_type(), ExchangeType::Ack);
    }

    #[test]
    fn test_parse_incomplete() {
        let data = [0xFF];
        let result = Pi11::parse(&data);
        assert!(result.is_err());
    }
}
