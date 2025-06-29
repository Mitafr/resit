use nom::IResult;

use crate::protocol::pi::Pi;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Pi4(pub [u8; 24]);

impl Pi for Pi4 {
    fn parse(data: &[u8]) -> IResult<&[u8], Self> {
        let (data, bytes) = nom::bytes::complete::take(24usize)(data)?;
        Ok((data, Pi4(bytes.try_into().unwrap())))
    }
}
