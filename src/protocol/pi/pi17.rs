use nom::IResult;

use crate::protocol::pi::Pi;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum TransferPriority {
    #[default]
    Urgent = 0,
    SemiUrgent = 1,
    LeastUrgent = 2,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Pi17(TransferPriority);

impl Pi for Pi17 {
    fn parse(data: &[u8]) -> IResult<&[u8], Self>
    where
        Self: Sized,
    {
        if data.is_empty() {
            Ok((data, Self(TransferPriority::default())))
        } else {
            let priority = match data[0] {
                0 => TransferPriority::Urgent,
                1 => TransferPriority::SemiUrgent,
                2 => TransferPriority::LeastUrgent,
                _ => {
                    return Err(nom::Err::Error(nom::error::Error::new(
                        data,
                        nom::error::ErrorKind::Tag,
                    )))
                }
            };
            Ok((&data[1..], Self(priority)))
        }
    }

    fn as_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(1);
        buf.push(self.0 as u8);
        buf
    }

    fn code(&self) -> u8 {
        17
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
    fn test_parse() {
        let data = vec![0];
        let (rem, pi) = Pi17::parse(&data).unwrap();
        assert_eq!(rem, &data[1..]);
        assert_eq!(pi.0, TransferPriority::Urgent);
        let data = vec![1];
        let (rem, pi) = Pi17::parse(&data).unwrap();
        assert_eq!(rem, &data[1..]);
        assert_eq!(pi.0, TransferPriority::SemiUrgent);
        let data = vec![2];
        let (rem, pi) = Pi17::parse(&data).unwrap();
        assert_eq!(rem, &data[1..]);
        assert_eq!(pi.0, TransferPriority::LeastUrgent);
    }

    #[test]
    fn test_parse_incomplete() {
        let data = vec![];
        let (rem, pi) = Pi17::parse(&data).unwrap();
        assert_eq!(rem, &data[..]);
        assert_eq!(pi.0, TransferPriority::default());
    }

    #[test]
    fn test_parse_invalid() {
        let data = vec![3];
        let result = Pi17::parse(&data);
        assert!(result.is_err());
    }
}
