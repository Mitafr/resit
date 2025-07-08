use nom::IResult;

use crate::protocol::pi::Pi;

pub(crate) mod connect;
pub(crate) mod create;
pub(crate) mod prelude;
pub(crate) mod release;

pub(crate) fn parse_pi<P: Pi>(input: &[u8]) -> IResult<&[u8], P> {
    P::parse(input)
}
