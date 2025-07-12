use log::warn;
use nom::IResult;

use crate::protocol::pi::Pi;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub(crate) enum UseOfSignature {
    #[default]
    NoSignature = 0,
    FileSigned = 1,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Pi34(pub UseOfSignature);

impl Pi for Pi34 {
    fn parse(data: &[u8]) -> IResult<&[u8], Self>
    where
        Self: Sized,
    {
        let (data, version) = nom::number::complete::u8(data)?;
        let version = match version {
            0 => UseOfSignature::NoSignature,
            1 => UseOfSignature::FileSigned,
            _ => {
                warn!("Unknown Pi34 version: {version}");
                UseOfSignature::default()
            }
        };
        Ok((data, Pi34(version)))
    }

    fn as_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(1);
        buf.push(self.0 as u8);
        buf
    }

    fn code(&self) -> u8 {
        34
    }

    fn len(&self) -> usize {
        2
    }

    fn ptype(&self) -> super::PiType {
        super::PiType::N
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_no_signature() {
        let data = [0];
        let result = Pi34::parse(&data);
        assert!(result.is_ok());
        let (_, pi34) = result.unwrap();
        assert_eq!(pi34, Pi34(UseOfSignature::NoSignature));
    }

    #[test]
    fn test_parse_file_signed() {
        let data = [1];
        let result = Pi34::parse(&data);
        assert!(result.is_ok());
        let (_, pi34) = result.unwrap();
        assert_eq!(pi34, Pi34(UseOfSignature::FileSigned));
    }

    #[test]
    fn test_parse_unknown() {
        let data = [3];
        let result = Pi34::parse(&data);
        assert!(result.is_ok());
        let (_, pi34) = result.unwrap();
        assert_eq!(pi34, Pi34(UseOfSignature::default()));
    }
}
