use nom::IResult;

pub(crate) mod pi1;
pub(crate) mod pi2;
pub(crate) mod pi3;
pub(crate) mod pi4;
pub(crate) mod pi5;
pub(crate) mod pi6;
pub(crate) mod pi7;
pub(crate) mod pi71;
pub(crate) mod pi91;
pub(crate) mod pi99;

pub(crate) use pi1::Pi1;
pub(crate) use pi2::Pi2;
pub(crate) use pi3::Pi3;
pub(crate) use pi4::Pi4;
pub(crate) use pi5::Pi5;
pub(crate) use pi6::Pi6;
pub(crate) use pi7::Pi7;
pub(crate) use pi71::Pi71;
pub(crate) use pi91::Pi91;
pub(crate) use pi99::Pi99;

pub trait Pi: Default {
    fn parse(data: &[u8]) -> IResult<&[u8], Self>
    where
        Self: Sized;
}
