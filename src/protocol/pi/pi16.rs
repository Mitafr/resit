use nom::IResult;

use crate::protocol::pi::Pi;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum DataCoding {
    #[default]
    Ascii = 0,
    Ebcdic = 1,
    Binary = 2,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Pi16(DataCoding);

impl Pi for Pi16 {
    fn parse(data: &[u8]) -> IResult<&[u8], Self>
    where
        Self: Sized,
    {
        if data.is_empty() {
            Ok((data, Self(DataCoding::default())))
        } else {
            let coding = match data[0] {
                0 => DataCoding::Ascii,
                1 => DataCoding::Ebcdic,
                2 => DataCoding::Binary,
                _ => {
                    return Err(nom::Err::Error(nom::error::Error::new(
                        data,
                        nom::error::ErrorKind::Tag,
                    )))
                }
            };
            Ok((&data[1..], Self(coding)))
        }
    }

    fn as_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(1);
        buf.push(self.0 as u8);
        buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let data = vec![0, 0, 0];
        let (rem, pi) = Pi16::parse(&data).unwrap();
        assert_eq!(rem, &data[1..]);
        assert_eq!(pi.0, DataCoding::Ascii);
    }
}
