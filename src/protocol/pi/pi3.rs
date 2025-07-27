use nom::IResult;

use crate::protocol::pi::{Pi, PiAsBytes, PiDefinition};

/// Pi3 is a structure that holds the caller ID
/// This protocol identifier is optional except for the FPDU.CONNECT
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Pi3(pub [u8; 24]);

impl Pi for Pi3 {
    fn parse(data: &[u8]) -> IResult<&[u8], Self> {
        let (data, bytes) = nom::bytes::complete::take(24usize)(data)?;
        Ok((data, Pi3(bytes.try_into().unwrap())))
    }
}

impl PiAsBytes for Pi3 {
    fn as_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(1 + 24);
        buf.push(Self::code());
        buf.extend(&self.0);
        buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let data = vec![0; 24];
        let (rem, pi) = Pi3::parse(&data).unwrap();
        assert_eq!(rem, &data[24..]);
        assert_eq!(pi.0, [0; 24]);
    }

    #[test]
    fn test_parse_incomplete() {
        let data = vec![0; 23];
        let result = Pi3::parse(&data);
        assert!(result.is_err());
    }

    #[test]
    fn test_as_bytes() {
        let pi3 = Pi3([0; 24]);
        let mut excepted = vec![3];
        excepted.extend([0u8; 24]);
        assert_eq!(excepted, pi3.as_bytes());
    }
}
