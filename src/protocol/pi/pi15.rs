use nom::IResult;

use crate::protocol::pi::Pi;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum Recovered {
    #[default]
    New = 0,
    Recovered = 1,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Pi15(Recovered);

impl Pi for Pi15 {
    fn parse(data: &[u8]) -> IResult<&[u8], Self>
    where
        Self: Sized,
    {
        if data.len() < 1 {
            Ok((&data[..], Self(Recovered::default())))
        } else {
            let recovered = match data[0] {
                0 => Recovered::New,
                1 => Recovered::Recovered,
                _ => {
                    return Err(nom::Err::Error(nom::error::Error::new(
                        data,
                        nom::error::ErrorKind::Tag,
                    )))
                }
            };
            Ok((&data[1..], Self(recovered)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let data = vec![0, 0, 0];
        let (rem, pi) = Pi15::parse(&data).unwrap();
        assert_eq!(rem, &data[1..]);
        assert_eq!(pi.0, Recovered::New);
    }
}
