use log::warn;
use nom::IResult;

use crate::protocol::pi::Pi;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub(crate) enum ArticleFormat {
    #[default]
    Fixed = 0,
    Variable = 0x80,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Pi31(pub ArticleFormat);

impl Pi for Pi31 {
    fn parse(data: &[u8]) -> IResult<&[u8], Self>
    where
        Self: Sized,
    {
        let (data, version) = nom::number::complete::u8(data)?;
        let version = match version {
            0 => ArticleFormat::Fixed,
            0x80 => ArticleFormat::Variable,
            _ => {
                warn!("Unknown Pi31 version: {version}");
                ArticleFormat::default()
            }
        };
        Ok((data, Pi31(version)))
    }

    fn as_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(1);
        buf.push(self.0 as u8);
        buf
    }

    fn code(&self) -> u8 {
        31
    }

    fn len(&self) -> usize {
        1
    }

    fn ptype(&self) -> super::PiType {
        super::PiType::M
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_fixed() {
        let data = [0];
        let result = Pi31::parse(&data);
        assert!(result.is_ok());
        let (_, pi31) = result.unwrap();
        assert_eq!(pi31, Pi31(ArticleFormat::Fixed));
    }

    #[test]
    fn test_parse_variable() {
        let data = [0x80];
        let result = Pi31::parse(&data);
        assert!(result.is_ok());
        let (_, pi31) = result.unwrap();
        assert_eq!(pi31, Pi31(ArticleFormat::Variable));
    }

    #[test]
    fn test_parse_unknown() {
        let data = [1];
        let result = Pi31::parse(&data);
        assert!(result.is_ok());
        let (_, pi31) = result.unwrap();
        assert_eq!(pi31, Pi31(ArticleFormat::default()));
    }
}
