use nom::IResult;

use crate::protocol::pi::Pi;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Pi51(pub [u8; 12]);

impl Pi for Pi51 {
    fn parse(data: &[u8]) -> IResult<&[u8], Self>
    where
        Self: Sized,
    {
        if data.len() < 12 {
            Err(nom::Err::Incomplete(nom::Needed::new(12)))
        } else {
            let value = [
                data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7], data[8],
                data[9], data[10], data[11],
            ];
            Ok((&data[12..], Self(value)))
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
        let data = vec![0; 12];
        let (rem, pi) = Pi51::parse(&data).unwrap();
        assert_eq!(rem, &data[12..]);
        assert_eq!(pi.0, [0; 12]);
    }

    #[test]
    fn test_parse_incomplete() {
        let data = vec![0; 11];
        let result = Pi51::parse(&data);
        assert!(result.is_err());
    }
}
