use nom::IResult;

use crate::protocol::pi::Pi;

/// Pi3 is a structure that holds the caller ID
/// This protocol identifier is optional except for the FPDU.CONNECT
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Pi3(pub [u8; 24]);

impl Pi for Pi3 {
    fn parse(data: &[u8]) -> IResult<&[u8], Self> {
        let (data, bytes) = nom::bytes::complete::take(24usize)(data)?;
        Ok((data, Pi3(bytes.try_into().unwrap())))
    }

    fn as_bytes(&self) -> Vec<u8> {
        self.0.to_vec()
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
}
