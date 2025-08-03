use nom::IResult;

use crate::protocol::pi::{Pi, PiAsBytes, PiDefinition};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Pi42([u8; 4]);

impl Pi for Pi42 {
    fn parse(data: &[u8]) -> IResult<&[u8], Self>
    where
        Self: Sized,
    {
        if data.len() < 4 {
            Err(nom::Err::Incomplete(nom::Needed::new(4)))
        } else {
            let value = [data[0], data[1], data[2], data[3]];
            Ok((&data[4..], Self(value)))
        }
    }
}

impl PiAsBytes for Pi42 {
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
        let data = vec![0, 0, 0, 0];
        let (rem, pi) = Pi42::parse(&data).unwrap();
        assert_eq!(rem, &data[4..]);
        assert_eq!(pi.0, [0, 0, 0, 0]);
    }

    #[test]
    fn test_parse_incomplete() {
        let data = vec![0, 0, 0];
        let result = Pi42::parse(&data);
        assert!(result.is_err());
    }
}
