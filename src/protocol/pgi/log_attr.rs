use nom::IResult;

use crate::protocol::{
    handler::fpdu::parse_pi,
    pi::{pi31::Pi31, pi32::Pi32, pi33::Pi33, pi34::Pi34, pi37::Pi37, PiAsBytes},
};

use super::Pgi;

#[derive(Default, Debug, bon::Builder)]
pub(crate) struct LogicalAttribute {
    #[builder(default)]
    len: u8,
    #[builder(default)]
    pis: LogicalAttributePis,
}

#[derive(Default, Debug, bon::Builder)]
pub(crate) struct LogicalAttributePis {
    #[builder(default)]
    pi31: Pi31,
    #[builder(default)]
    pi32: Pi32,
    #[builder(default)]
    pi33: Pi33,
    #[builder(default)]
    pi34: Pi34,
    #[builder(default)]
    pi37: Pi37,
    /* pi38: Pi38,
    pi39: Pi39,*/
}

impl Pgi for LogicalAttribute {
    fn parse(data: &[u8]) -> IResult<&[u8], Self> {
        let (data, len) = Self::parse_len(&data[1..])?;
        let (data, pi31) = parse_pi::<Pi31>(data).unwrap();
        let (data, pi32) = parse_pi::<Pi32>(data).unwrap();
        let (data, pi33) = parse_pi::<Pi33>(data).unwrap();
        let (data, pi34) = parse_pi::<Pi34>(data).unwrap();
        let (data, pi37) = parse_pi::<Pi37>(data).unwrap();
        Ok((
            data,
            Self {
                len,
                pis: LogicalAttributePis::builder()
                    .pi31(pi31)
                    .pi32(pi32)
                    .pi33(pi33)
                    .pi34(pi34)
                    .pi37(pi37)
                    .build(),
            },
        ))
    }

    fn handle(&self) {
        todo!()
    }

    fn code() -> u8 {
        30
    }
}

impl PiAsBytes for LogicalAttribute {
    fn as_bytes(&self) -> Vec<u8> {
        let mut buf = vec![Self::code(), self.len];
        buf.extend(self.pis.pi31.as_bytes());
        buf.extend(self.pis.pi32.as_bytes());
        buf.extend(self.pis.pi33.as_bytes());
        buf.extend(self.pis.pi34.as_bytes());
        buf.extend(self.pis.pi37.as_bytes());
        buf
    }
}
