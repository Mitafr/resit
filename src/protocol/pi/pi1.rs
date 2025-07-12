use nom::{bytes::complete::take, IResult};

use crate::protocol::pi::Pi;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Pi1(pub bool);

impl Pi for Pi1 {
    fn parse(data: &[u8]) -> IResult<&[u8], Self> {
        let (data, bytes) = take(1usize)(data)?;
        Ok((data, Pi1(bytes[0] == 1)))
    }

    fn as_bytes(&self) -> Vec<u8> {
        vec![if self.0 { 1 } else { 0 }]
    }

    fn code(&self) -> u8 {
        1
    }

    fn len(&self) -> usize {
        1
    }

    fn ptype(&self) -> super::PiType {
        super::PiType::S
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_pi1() {
        let input = [0x00, 0x00, 0x00, 0x00, 0x01];
        let result = Pi1::parse(&input);
        assert!(result.is_ok());
        let (remain, pi1) = result.unwrap();
        assert_eq!(pi1, Pi1(false));
        assert_eq!(remain, &input[1..]);
    }
}
