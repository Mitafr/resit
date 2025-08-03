use nom::IResult;

use crate::protocol::pi::{Pi, PiAsBytes, PiDefinition};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Pi38(pub [u8; 2]);

impl Pi for Pi38 {
    fn parse(data: &[u8]) -> IResult<&[u8], Self>
    where
        Self: Sized,
    {
        let (data, signature) = nom::bytes::complete::take(2usize)(data)?;
        Ok((data, Pi38(signature.try_into().unwrap())))
    }
}

impl PiAsBytes for Pi38 {
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
        let data = [0; 2];
        let result = Pi38::parse(&data);
        assert!(result.is_ok());
        let (_, pi38) = result.unwrap();
        assert_eq!(pi38, Pi38([0; 2]));
    }
}
