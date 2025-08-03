use nom::IResult;

use crate::protocol::pi::{Pi, PiAsBytes, PiDefinition};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum AccessType {
    #[default]
    Write = 0,
    Read = 1,
    Mix = 2,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Pi22(AccessType);

impl Pi for Pi22 {
    fn parse(data: &[u8]) -> IResult<&[u8], Self>
    where
        Self: Sized,
    {
        if data.is_empty() {
            Ok((data, Self(AccessType::default())))
        } else {
            let access_type = match data[0] {
                0 => AccessType::Write,
                1 => AccessType::Read,
                2 => AccessType::Mix,
                _ => {
                    return Err(nom::Err::Error(nom::error::Error::new(
                        data,
                        nom::error::ErrorKind::Tag,
                    )))
                }
            };
            Ok((&data[1..], Self(access_type)))
        }
    }
}

impl PiAsBytes for Pi22 {
    fn as_bytes(&self) -> Vec<u8> {
        vec![Self::code(), self.0 as u8]
    }
}
