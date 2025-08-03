
use nom::IResult;

use crate::protocol::pi::{Pi, PiAsBytes, PiDefinition};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Pi61(pub [u8; 24]);

impl Pi for Pi61 {
    fn parse(data: &[u8]) -> IResult<&[u8], Self>
    where
        Self: Sized,
    {
        if data.len() < 24 {
            Err(nom::Err::Incomplete(nom::Needed::new(24)))
        } else {
            Ok((&data[24..], Self(data[..24].try_into().unwrap_or_default())))
        }
    }
}

impl PiAsBytes for Pi61 {
    fn as_bytes(&self) -> Vec<u8> {
        let mut buf = vec![Self::code()];
        buf.extend(self.0);
        buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let data = vec![0; 24];
        let (rem, pi) = Pi61::parse(&data).unwrap();
        assert_eq!(rem, &data[24..]);
        assert_eq!(pi.0, [0; 24]);
    }

    #[test]
    fn test_parse_incomplete() {
        let data = vec![0; 23];
        let result = Pi61::parse(&data);
        assert!(result.is_err());
    }
}
