use nom::{bits::complete::take, IResult};

use crate::protocol::pi::Pi;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Pi1(pub bool);

impl Pi for Pi1 {
    fn parse(data: &[u8]) -> IResult<&[u8], Self> {
        log::info!("Parsing Pi1 from data: {:?}", data);
        match take::<&[u8], u8, usize, (_, nom::error::ErrorKind)>(1usize)((data, 7)) {
            Ok(((data, _), crc)) => Ok((data, Pi1(1u8 == crc))),
            Err(e) => {
                log::error!("Failed to parse Pi1: {}", e);
                Ok((data, Pi1(false)))
            }
        }
    }
}
