use nom::IResult;

use crate::protocol::pi::Pi;

#[derive(Debug, Clone, PartialEq)]
pub struct Pi36(pub [u8; 64]);

impl Default for Pi36 {
    fn default() -> Self {
        Pi36([0; 64])
    }
}

impl Pi for Pi36 {
    fn parse(_data: &[u8]) -> IResult<&[u8], Self>
    where
        Self: Sized,
    {
        unreachable!("Not Authorized")
    }

    fn as_bytes(&self) -> Vec<u8> {
        unreachable!("Not Authorized")
    }
}
