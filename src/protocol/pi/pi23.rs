use nom::IResult;

use crate::protocol::pi::{Pi, PiAsBytes, PiDefinition};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum Resync {
    #[default]
    Unauthorized = 0,
    Authorized = 1,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Pi23(Resync);

impl Pi for Pi23 {
    fn parse(data: &[u8]) -> IResult<&[u8], Self>
    where
        Self: Sized,
    {
        if data.is_empty() {
            Ok((data, Self(Resync::default())))
        } else {
            let resync = match data[0] {
                0 => Resync::Unauthorized,
                1 => Resync::Authorized,
                _ => {
                    return Err(nom::Err::Error(nom::error::Error::new(
                        data,
                        nom::error::ErrorKind::Tag,
                    )))
                }
            };
            Ok((&data[1..], Self(resync)))
        }
    }
}

impl PiAsBytes for Pi23 {
    fn as_bytes(&self) -> Vec<u8> {
        vec![Self::code(), self.0 as u8]
    }
}
