use nom::IResult;

use crate::protocol::pi::{Pi, PiAsBytes};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Pi13([u8; 3]);

impl Pi for Pi13 {
    fn parse(data: &[u8]) -> IResult<&[u8], Self>
    where
        Self: Sized,
    {
        if data.len() < 3 {
            return Err(nom::Err::Incomplete(nom::Needed::new(3)));
        }
        let mut array = [0u8; 3];
        array.copy_from_slice(&data[..3]);
        Ok((&data[3..], Self(array)))
    }
}

impl PiAsBytes for Pi13 {
    fn as_bytes(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let data = vec![0, 0, 0];
        let (rem, pi) = Pi13::parse(&data).unwrap();
        assert_eq!(rem, &data[3..]);
        assert_eq!(pi.0, [0, 0, 0]);
    }
}
