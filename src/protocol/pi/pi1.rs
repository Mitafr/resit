use nom::{bytes::complete::take, IResult};

use crate::protocol::pi::{Pi, PiAsBytes};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Pi1(pub bool);

impl Pi for Pi1 {
    fn parse(data: &[u8]) -> IResult<&[u8], Self> {
        let (data, bytes) = take(1usize)(data)?;
        Ok((data, Pi1(bytes[0] == 1)))
    }
}

impl PiAsBytes for Pi1 {
    fn as_bytes(&self) -> Vec<u8> {
        vec![u8::from(self.0)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_pi1_false() {
        let input = [0x00, 0x00, 0x00, 0x00, 0x01];
        let result = Pi1::parse(&input);
        assert!(result.is_ok());
        let (remain, pi1) = result.unwrap();
        assert_eq!(pi1, Pi1(false));
        assert_eq!(remain, &input[1..]);
        assert_eq!(pi1.as_bytes(), vec![0]);
    }

    #[test]
    fn test_parse_pi1_true() {
        let input = [0x01, 0x00, 0x00, 0x00, 0x01];
        let result = Pi1::parse(&input);
        assert!(result.is_ok());
        let (remain, pi1) = result.unwrap();
        assert_eq!(pi1, Pi1(true));
        assert_eq!(remain, &input[1..]);
        assert_eq!(pi1.as_bytes(), vec![1]);
    }
}
