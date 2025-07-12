use log::warn;
use nom::IResult;

use crate::protocol::pi::Pi;

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

    fn as_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(1);
        buf.push(self.0 as u8);
        buf
    }

    fn code(&self) -> u8 {
        6
    }

    fn len(&self) -> usize {
        2
    }

    fn ptype(&self) -> super::PiType {
        super::PiType::C
    }
}
