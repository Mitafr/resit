use nom::IResult;

use crate::protocol::pi::Pi;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Pi4(pub [u8; 24]);

impl Pi for Pi4 {
    fn parse(data: &[u8]) -> IResult<&[u8], Self> {
        let (data, bytes) = nom::bytes::complete::take(24usize)(data)?;
        Ok((data, Pi4(bytes.try_into().unwrap())))
    }

    fn as_bytes(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    fn code(&self) -> u8 {
        4
    }

    fn len(&self) -> usize {
        24
    }

    fn ptype(&self) -> super::PiType {
        super::PiType::C
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
}
