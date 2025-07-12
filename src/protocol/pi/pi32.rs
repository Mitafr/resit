use nom::IResult;

use crate::protocol::pi::Pi;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Pi32(u16);

impl Pi for Pi32 {
    fn parse(data: &[u8]) -> IResult<&[u8], Self>
    where
        Self: Sized,
    {
        if data.len() < 2 {
            Err(nom::Err::Incomplete(nom::Needed::new(2)))
        } else {
            let value = u16::from_le_bytes([data[0], data[1]]);
            Ok((&data[2..], Self(value)))
        }
    }

    fn as_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(2);
        buf.extend_from_slice(&self.0.to_le_bytes());
        buf
    }

    fn code(&self) -> u8 {
        32
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
    fn test_parse() {
        let data = vec![0, 0];
        let (rem, pi) = Pi32::parse(&data).unwrap();
        assert_eq!(rem, &data[2..]);
        assert_eq!(pi.0, 0);
    }

    #[test]
    fn test_parse_incomplete() {
        let data = vec![0];
        let result = Pi32::parse(&data);
        assert!(result.is_err());
    }
}
