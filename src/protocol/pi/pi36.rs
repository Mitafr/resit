use nom::IResult;

use crate::protocol::pi::Pi;

#[derive(Debug, Clone, PartialEq)]
pub struct Pi36(pub [u8; 64]);

impl Default for Pi36 {
    fn default() -> Self {
        Pi36([0; 64])
    }
}

impl Pi for Pi36 {
    fn parse(data: &[u8]) -> IResult<&[u8], Self>
    where
        Self: Sized,
    {
        let (data, signature) = nom::bytes::complete::take(64u8)(data)?;
        Ok((data, Pi36(signature.try_into().unwrap())))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let data = [0; 64];
        let result = Pi36::parse(&data);
        assert!(result.is_ok());
        let (_, pi36) = result.unwrap();
        assert_eq!(pi36, Pi36([0; 64]));
    }
}
