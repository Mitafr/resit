use log::warn;
use nom::IResult;

use crate::protocol::pi::{Pi, PiAsBytes, PiDefinition};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub(crate) enum StorageReservationUnit {
    #[default]
    Kbytes = 0,
    Articles = 1,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Pi41(pub StorageReservationUnit);

impl Pi for Pi41 {
    fn parse(data: &[u8]) -> IResult<&[u8], Self>
    where
        Self: Sized,
    {
        let (data, version) = nom::number::complete::u8(data)?;
        let version: StorageReservationUnit = match version {
            0 => StorageReservationUnit::Kbytes,
            1 => StorageReservationUnit::Articles,
            _ => {
                warn!("Unknown Pi41 version: {version}");
                StorageReservationUnit::default()
            }
        };
        Ok((data, Pi41(version)))
    }
}

impl PiAsBytes for Pi41 {
    fn as_bytes(&self) -> Vec<u8> {
        vec![Self::code(), self.0 as u8]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_no_signature() {
        let data = [0];
        let result = Pi41::parse(&data);
        assert!(result.is_ok());
        let (_, pi41) = result.unwrap();
        assert_eq!(pi41, Pi41(StorageReservationUnit::Kbytes));
    }

    #[test]
    fn test_parse_file_signed() {
        let data = [1];
        let result = Pi41::parse(&data);
        assert!(result.is_ok());
        let (_, pi41) = result.unwrap();
        assert_eq!(pi41, Pi41(StorageReservationUnit::Articles));
    }

    #[test]
    fn test_parse_unknown() {
        let data = [3];
        let result = Pi41::parse(&data);
        assert!(result.is_ok());
        let (_, pi41) = result.unwrap();
        assert_eq!(pi41, Pi41(StorageReservationUnit::default()));
    }
}
