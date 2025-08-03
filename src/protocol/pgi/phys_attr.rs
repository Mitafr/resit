use nom::IResult;

use crate::protocol::{
    handler::fpdu::parse_pi,
    pi::{pi41::Pi41, pi42::Pi42, PiAsBytes},
};

use super::Pgi;

#[derive(Default, Debug, bon::Builder)]
pub(crate) struct PhysicalAttribute {
    #[builder(default)]
    len: u8,
    #[builder(default)]
    pis: PhysicalAttributePis,
}

#[derive(Default, Debug, bon::Builder)]
pub(crate) struct PhysicalAttributePis {
    #[builder(default)]
    pi41: Pi41,
    #[builder(default)]
    pi42: Pi42,
}

impl Pgi for PhysicalAttribute {
    fn parse(data: &[u8]) -> IResult<&[u8], Self> {
        let (data, len) = Self::parse_len(&data[1..])?;
        let (data, pi41) = parse_pi::<Pi41>(data).unwrap();
        let (data, pi42) = parse_pi::<Pi42>(data).unwrap();
        Ok((
            data,
            Self {
                len,
                pis: PhysicalAttributePis::builder()
                    .pi41(pi41)
                    .pi42(pi42)
                    .build(),
            },
        ))
    }

    fn handle(&self) {
        todo!()
    }

    fn code() -> u8 {
        40
    }
}

impl PiAsBytes for PhysicalAttribute {
    fn as_bytes(&self) -> Vec<u8> {
        let mut buf = vec![Self::code(), self.len];
        buf.extend(self.pis.pi41.as_bytes());
        buf.extend(self.pis.pi42.as_bytes());
        buf
    }
}
