use log::warn;
use nom::IResult;

use crate::protocol::pi::{Pi, PiAsBytes, PiDefinition};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub(crate) enum Version {
    #[default]
    D = 1,
    E = 2,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Pi6(pub Version);

impl Pi for Pi6 {
    fn parse(data: &[u8]) -> IResult<&[u8], Self>
    where
        Self: Sized,
    {
        let (data, version) = nom::number::complete::u8(data)?;
        let version = match version {
            1 => Version::D,
            2 => Version::E,
            _ => {
                warn!("Unknown Pi6 version: {version}");
                Version::default()
            }
        };
        Ok((data, Pi6(version)))
    }
}

impl PiAsBytes for Pi6 {
    fn as_bytes(&self) -> Vec<u8> {
        vec![Self::code(), self.0 as u8]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let data = vec![1u8];
        let (remain, pi6) = Pi6::parse(&data).unwrap();
        assert!(remain.is_empty());
        assert_eq!(Version::D, pi6.0);

        let data = vec![2u8];
        let (remain, pi6) = Pi6::parse(&data).unwrap();
        assert!(remain.is_empty());
        assert_eq!(Version::E, pi6.0);
    }

    #[test]
    fn test_unknown_version() {
        let data = vec![0u8];
        let (remain, pi6) = Pi6::parse(&data).unwrap();
        assert!(remain.is_empty());
        assert_eq!(Version::D, pi6.0);
    }

    #[test]
    fn test_as_bytes() {
        let pi6 = Pi6(Version::E);
        let excepted = vec![6, 2];
        assert_eq!(excepted, pi6.as_bytes());
    }
}
