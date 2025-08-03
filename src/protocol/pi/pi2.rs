use nom::IResult;

use crate::protocol::pi::{Pi, PiAsBytes, PiDefinition};

pub enum Diag {
    Success,
    LocalCommunicationSaturation,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Pi2 {
    pub error_type: u8,
    pub reason_code: u16,
}

impl From<Diag> for Pi2 {
    fn from(value: Diag) -> Self {
        match value {
            Diag::Success => Pi2 {
                error_type: 0,
                reason_code: 0,
            },
            Diag::LocalCommunicationSaturation => Pi2 {
                error_type: 3,
                reason_code: 300,
            },
            _ => Pi2 {
                error_type: 0,
                reason_code: 0,
            },
        }
    }
}

impl Pi for Pi2 {
    fn parse(data: &[u8]) -> IResult<&[u8], Self> {
        let (data, error_type) = nom::number::complete::u8(data)?;
        let (data, reason_code) =
            nom::number::complete::u16(nom::number::Endianness::Native)(data)?;
        Ok((
            data,
            Pi2 {
                error_type,
                reason_code,
            },
        ))
    }
}

impl PiAsBytes for Pi2 {
    fn as_bytes(&self) -> Vec<u8> {
        let mut buf = vec![Self::code()];
        buf.push(self.error_type);
        buf.extend_from_slice(&self.reason_code.to_be_bytes());
        buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pi2_parse() {
        let data = [0u8, 0xFF, 0xF0];
        let (remain_, pi2) = Pi2::parse(&data).unwrap();
        assert_eq!(pi2.error_type, 0);
        assert_eq!(pi2.reason_code, 0xF0FF);
        assert_eq!(
            pi2,
            Pi2 {
                error_type: 0,
                reason_code: 0xF0FF
            }
        );
        assert!(remain_.is_empty());
        assert_eq!(pi2.as_bytes(), vec![2, 0u8, 0xF0, 0xFF]);
    }

    #[test]
    fn test_as_bytes() {
        let pi2 = Pi2 {
            error_type: 0,
            reason_code: 0,
        };
        let mut excepted = vec![2];
        excepted.extend([0u8; 3]);
        assert_eq!(excepted, pi2.as_bytes());
    }
}
