use nom::IResult;

pub(crate) mod pi1;
pub(crate) mod pi11;
pub(crate) mod pi12;
pub(crate) mod pi13;
pub(crate) mod pi15;
pub(crate) mod pi16;
pub(crate) mod pi17;
pub(crate) mod pi2;
pub(crate) mod pi25;
pub(crate) mod pi3;
pub(crate) mod pi4;
pub(crate) mod pi5;
pub(crate) mod pi6;
pub(crate) mod pi7;
pub(crate) mod pi71;
pub(crate) mod pi91;
pub(crate) mod pi99;

pub(crate) use pi1::Pi1;
pub(crate) use pi11::Pi11;
pub(crate) use pi12::Pi12;
pub(crate) use pi13::Pi13;
pub(crate) use pi15::Pi15;
pub(crate) use pi16::Pi16;
pub(crate) use pi17::Pi17;
pub(crate) use pi2::Pi2;
pub(crate) use pi25::Pi25;
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
