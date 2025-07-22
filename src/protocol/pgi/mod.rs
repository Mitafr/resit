use nom::IResult;

use crate::protocol::pi::{
    pi11::Pi11, pi12::Pi12, pi3::Pi3, pi31::Pi31, pi32::Pi32, pi33::Pi33, pi34::Pi34, pi36::Pi36,
    pi37::Pi37, pi4::Pi4, pi41::Pi41, pi42::Pi42, pi51::Pi51, pi52::Pi52,
};

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum PgiType {
    FileDescriptor(Pi3, Pi4, Pi11, Pi12),
    LogicalAttr(Pi31, Pi32, Pi33, Pi34, Pi36, Pi37 /* Pi38, Pi39 */),
    PhysicalAttr(Pi41, Pi42),
    HistoricAttr(Pi51, Pi52),
}

#[derive(Default, Debug)]
pub(crate) struct PgiFileDescriptor {}
#[derive(Default, Debug)]
pub(crate) struct PgiFileDescriptorPis {
    pi3: Pi3,
    pi4: Pi4,
    pi11: Pi11,
    pi12: Pi12,
}

impl Pgi for PgiFileDescriptor {
    type Pis = PgiFileDescriptorPis;

    fn parse(data: &[u8]) -> IResult<&[u8], Self::Pis> {
        let (data, code) = Self::parse_code(data)?;
        let (data, len) = Self::parse_len(data)?;
        Ok((data, Self::Pis::default()))
    }

    fn handle(&self) {
        todo!()
    }
}

pub trait Pgi: Default {
    type Pis;

    fn parse(data: &[u8]) -> IResult<&[u8], Self::Pis>;

    fn parse_code(data: &[u8]) -> IResult<&[u8], u8> {
        let (data, code) = nom::bytes::complete::take(1usize)(data)?;
        Ok((data, code[0]))
    }

    fn parse_len(data: &[u8]) -> IResult<&[u8], u8> {
        let (data, len) = nom::bytes::complete::take(1usize)(data)?;
        Ok((data, len[0]))
    }

    fn handle(&self);
}
