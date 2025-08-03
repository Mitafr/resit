use nom::IResult;

use crate::protocol::{
    handler::fpdu::parse_pi,
    pi::{pi11::Pi11, pi12::Pi12, pi3::Pi3, pi4::Pi4, PiAsBytes},
};

use super::Pgi;

#[derive(Default, Debug, bon::Builder)]
pub(crate) struct FileDescriptor {
    #[builder(default)]
    pub len: u8,
    #[builder(default)]
    pub pis: FileDescriptorPis,
}

#[derive(Default, Debug, bon::Builder)]
pub(crate) struct FileDescriptorPis {
    #[builder(default)]
    pub pi3: Pi3,
    #[builder(default)]
    pub pi4: Pi4,
    #[builder(default)]
    pub pi11: Pi11,
    #[builder(default)]
    pub pi12: Pi12,
}

impl Pgi for FileDescriptor {
    fn parse(data: &[u8]) -> IResult<&[u8], Self> {
        let (data, len) = Self::parse_len(&data[1..])?;
        let (data, pi3) = parse_pi::<Pi3>(data).unwrap();
        let (data, pi4) = parse_pi::<Pi4>(data).unwrap();
        let (data, pi11) = parse_pi::<Pi11>(data).unwrap();
        let (data, pi12) = parse_pi::<Pi12>(data).unwrap();
        Ok((
            data,
            Self {
                len,
                pis: FileDescriptorPis::builder()
                    .pi3(pi3)
                    .pi4(pi4)
                    .pi11(pi11)
                    .pi12(pi12)
                    .build(),
            },
        ))
    }

    fn handle(&self) {
        todo!()
    }

    fn code() -> u8 {
        9
    }
}

impl PiAsBytes for FileDescriptor {
    fn as_bytes(&self) -> Vec<u8> {
        let mut buf = vec![Self::code(), self.len];
        buf.extend(self.pis.pi3.as_bytes());
        buf.extend(self.pis.pi4.as_bytes());
        buf.extend(self.pis.pi11.as_bytes());
        buf.extend(self.pis.pi12.as_bytes());
        buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_code() {
        let data = &[0x01, 0x02];
        let (remaining, code) = FileDescriptor::parse_code(data).unwrap();
        assert_eq!(code, 0x01);
        assert_eq!(remaining, &[0x02]);
    }

    #[test]
    fn test_parse_len() {
        let data = &[0x01, 0x02];
        let (remaining, len) = FileDescriptor::parse_len(data).unwrap();
        assert_eq!(len, 0x01);
        assert_eq!(remaining, &[0x02]);
    }
}
