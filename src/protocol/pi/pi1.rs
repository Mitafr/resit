use nom::IResult;

use crate::protocol::pi::Pi;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Pi1(pub bool);

impl Pi for Pi1 {
    fn parse(data: &[u8]) -> IResult<&[u8], Self> {
        if let Ok(((data, _), crc)) =
            nom::bits::complete::bool::<_, nom::error::Error<(&[u8], usize)>>((data, 1usize))
        {
            Ok((data, Pi1(crc)))
        } else {
            Ok((data, Pi1(false)))
        }
    }
}
