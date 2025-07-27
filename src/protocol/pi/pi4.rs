use nom::IResult;

use crate::protocol::pi::{Pi, PiAsBytes, PiDefinition};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Pi4(pub [u8; 24]);

impl Pi for Pi4 {
    fn parse(data: &[u8]) -> IResult<&[u8], Self> {
        let (data, bytes) = nom::bytes::complete::take(24usize)(data)?;
        Ok((data, Pi4(bytes.try_into().unwrap())))
    }
}

impl PiAsBytes for Pi4 {
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
    fn test_pi4_parse() {
        let mut data = vec![];
        data.extend(0u8..=23u8);
        let (remain, pi4) = Pi4::parse(&data).unwrap();
        assert_eq!(pi4.0, *data);
        assert!(remain.is_empty());
    }

    #[test]
    fn test_as_bytes() {
        let pi4 = Pi4([0; 24]);
        let mut excepted = vec![4];
        excepted.extend([0u8; 24]);
        assert_eq!(excepted, pi4.as_bytes());
    }
}
