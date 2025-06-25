use nom::IResult;

pub(crate) mod pi1;
pub(crate) mod pi71;
pub(crate) mod pi91;
pub(crate) mod pi99;

pub trait Pi {
    fn parse(data: &[u8]) -> IResult<&[u8], Self>
    where
        Self: Sized;
}
