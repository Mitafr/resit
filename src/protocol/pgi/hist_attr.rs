use nom::IResult;

use crate::protocol::{
    handler::fpdu::parse_pi,
    pi::{pi51::Pi51, pi52::Pi52, PiAsBytes},
};

use super::Pgi;

#[derive(Default, Debug, bon::Builder)]
pub(crate) struct HistoricAttribute {
    #[builder(default)]
    len: u8,
    #[builder(default)]
    pis: HistoricAttributePis,
}

#[derive(Default, Debug, bon::Builder)]
pub(crate) struct HistoricAttributePis {
    #[builder(default)]
    pi51: Pi51,
    #[builder(default)]
    pi52: Pi52,
}

impl Pgi for HistoricAttribute {
    fn parse(data: &[u8]) -> IResult<&[u8], Self> {
        let (data, len) = Self::parse_len(&data[1..])?;
        let (data, pi51) = parse_pi::<Pi51>(data).unwrap();
        let (data, pi52) = parse_pi::<Pi52>(data).unwrap();
        Ok((
            data,
            Self {
                len,
                pis: HistoricAttributePis::builder()
                    .pi51(pi51)
                    .pi52(pi52)
                    .build(),
            },
        ))
    }

    fn handle(&self) {
        todo!()
    }

    fn code() -> u8 {
        50
    }
}

impl PiAsBytes for HistoricAttribute {
    fn as_bytes(&self) -> Vec<u8> {
        let mut buf = vec![Self::code(), self.len];
        buf.extend(self.pis.pi51.as_bytes());
        buf.extend(self.pis.pi52.as_bytes());
        buf
    }
}
