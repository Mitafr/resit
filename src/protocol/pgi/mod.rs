use nom::IResult;

use super::pi::PiAsBytes;

pub(crate) mod file_desc;
pub(crate) mod hist_attr;
pub(crate) mod log_attr;
pub(crate) mod phys_attr;

pub trait Pgi: Default + PiAsBytes {
    fn parse(data: &[u8]) -> IResult<&[u8], Self>;

    fn parse_code(data: &[u8]) -> IResult<&[u8], u8> {
        let (data, code) = nom::bytes::complete::take(1usize)(data)?;
        Ok((data, code[0]))
    }

    fn parse_len(data: &[u8]) -> IResult<&[u8], u8> {
        let (data, len) = nom::bytes::complete::take(1usize)(data)?;
        Ok((data, len[0]))
    }

    fn handle(&self);

    fn code() -> u8;
}
