use nom::IResult;

use crate::protocol::pi::Pi;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Pi62(pub [u8; 24]);

impl Pi for Pi62 {
    fn parse(data: &[u8]) -> IResult<&[u8], Self>
    where
        Self: Sized,
    {
        if data.len() < 24 {
            Err(nom::Err::Incomplete(nom::Needed::new(24)))
        } else {
            let value = [
                data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7], data[8],
                data[9], data[10], data[11], data[12], data[13], data[14], data[15], data[16],
                data[17], data[18], data[19], data[20], data[21], data[22], data[23],
            ];
            Ok((&data[24..], Self(value)))
        }
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
        let (rem, pi) = Pi62::parse(&data).unwrap();
        assert_eq!(rem, &data[24..]);
        assert_eq!(pi.0, [0; 24]);
    }

    #[test]
    fn test_parse_incomplete() {
        let data = vec![0; 23];
        let result = Pi62::parse(&data);
        assert!(result.is_err());
    }
}
