use nom::IResult;

use crate::protocol::pi::Pi;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Pi2 {
    error_type: u8,
    reason_code: u16,
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
