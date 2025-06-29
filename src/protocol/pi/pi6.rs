use log::warn;
use nom::IResult;

use crate::protocol::pi::Pi;

#[derive(Debug, Default, Clone, PartialEq)]
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
