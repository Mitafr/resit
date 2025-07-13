use nom::IResult;

use crate::protocol::pi::Pi;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum RequestedAttr {
    Logical,
    Physical,
    Historical,
    #[default]
    None,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Pi14(RequestedAttr);

impl Pi for Pi14 {
    fn parse(data: &[u8]) -> IResult<&[u8], Self>
    where
        Self: Sized,
    {
        if data.is_empty() {
            return Err(nom::Err::Incomplete(nom::Needed::new(1)));
        }
        let (rest, first_byte_slice) = nom::bytes::complete::take(1usize)(data)?;
        let byte = first_byte_slice[0];

        let attr = match byte & 0b0000_0111 {
            0b0000_0001 => RequestedAttr::Logical,
            0b0000_0010 => RequestedAttr::Physical,
            0b0000_0100 => RequestedAttr::Historical,
            0 => RequestedAttr::None,
            _ => {
                return Err(nom::Err::Failure(nom::error::Error::new(
                    data,
                    nom::error::ErrorKind::Alt,
                )));
            }
        };
        Ok((rest, Pi14(attr)))
    }

    fn as_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(1);
        buf.push(self.0 as u8);
        buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_none() {
        let input = [0x00];
        let (_, pi) = Pi14::parse(&input).unwrap();
        assert_eq!(pi.0, RequestedAttr::None);
    }

    #[test]
    fn test_parse_logical() {
        let input = [0x01];
        let (_, pi) = Pi14::parse(&input).unwrap();
        assert_eq!(pi.0, RequestedAttr::Logical);
    }

    #[test]
    fn test_parse_physical() {
        let input = [0x02];
        let (_, pi) = Pi14::parse(&input).unwrap();
        assert_eq!(pi.0, RequestedAttr::Physical);
    }

    #[test]
    fn test_parse_historical() {
        let input = [0x04];
        let (_, pi) = Pi14::parse(&input).unwrap();
        assert_eq!(pi.0, RequestedAttr::Historical);
    }

    #[test]
    fn test_invalid_combination_multiple_bits_set() {
        let input = [0x03];
        let result = Pi14::parse(&input);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_all_bits_set() {
        let input = [0xFF];
        let result = Pi14::parse(&input);
        assert!(result.is_err());
    }

    #[test]
    fn test_incomplete_input() {
        let input: &[u8] = &[];
        let result = Pi14::parse(input);
        assert!(matches!(result, Err(nom::Err::Incomplete(_))));
    }
}
