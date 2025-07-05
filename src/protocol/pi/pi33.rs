use log::warn;
use nom::IResult;

use crate::protocol::pi::Pi;

#[derive(Debug, Default, Clone, PartialEq)]
pub(crate) enum FileAttributes {
    #[default]
    Sequential = 0,
    Relative = 1,
    Indexed = 2,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Pi33(pub FileAttributes);

impl Pi for Pi33 {
    fn parse(data: &[u8]) -> IResult<&[u8], Self>
    where
        Self: Sized,
    {
        let (data, version) = nom::number::complete::u8(data)?;
        let version = match version {
            0 => FileAttributes::Sequential,
            1 => FileAttributes::Relative,
            2 => FileAttributes::Indexed,
            _ => {
                warn!("Unknown Pi33 version: {version}");
                FileAttributes::default()
            }
        };
        Ok((data, Pi33(version)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_fixed() {
        let data = [0];
        let result = Pi33::parse(&data);
        assert!(result.is_ok());
        let (_, pi33) = result.unwrap();
        assert_eq!(pi33, Pi33(FileAttributes::Sequential));
    }

    #[test]
    fn test_parse_variable() {
        let data = [1];
        let result = Pi33::parse(&data);
        assert!(result.is_ok());
        let (_, pi33) = result.unwrap();
        assert_eq!(pi33, Pi33(FileAttributes::Relative));
    }

    #[test]
    fn test_parse_indexed() {
        let data = [2];
        let result = Pi33::parse(&data);
        assert!(result.is_ok());
        let (_, pi33) = result.unwrap();
        assert_eq!(pi33, Pi33(FileAttributes::Indexed));
    }

    #[test]
    fn test_parse_unknown() {
        let data = [3];
        let result = Pi33::parse(&data);
        assert!(result.is_ok());
        let (_, pi33) = result.unwrap();
        assert_eq!(pi33, Pi33(FileAttributes::default()));
    }
}
