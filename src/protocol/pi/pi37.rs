use nom::IResult;

use crate::protocol::pi::{Pi, PiAsBytes, PiDefinition};

#[derive(Debug, Clone, PartialEq)]
pub struct Pi37(pub [u8; 80]);

impl Default for Pi37 {
    fn default() -> Self {
        Pi37([0; 80])
    }
}

impl Pi for Pi37 {
    fn parse(data: &[u8]) -> IResult<&[u8], Self>
    where
        Self: Sized,
    {
        let (data, signature) = nom::bytes::complete::take(80u8)(data)?;
        Ok((data, Pi37(signature.try_into().unwrap())))
    }
}

impl PiAsBytes for Pi37 {
    fn as_bytes(&self) -> Vec<u8> {
        let mut buf = vec![Self::code()];
        buf.extend(self.0);
        buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let data = [0; 80];
        let result = Pi37::parse(&data);
        assert!(result.is_ok());
        let (_, pi37) = result.unwrap();
        assert_eq!(pi37, Pi37([0; 80]));
    }
}
