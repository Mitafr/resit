use crate::protocol::pi::{Pi, PiAsBytes, PiDefinition};

#[derive(Debug, Clone, PartialEq)]
pub struct Pi99(pub [u8; 254]);

impl Default for Pi99 {
    fn default() -> Self {
        Pi99([0; 254])
    }
}

impl Pi99 {
    pub fn from_str(s: &str) -> Self {
        let bytes = s.as_bytes();
        let mut array = [0u8; 254];
        if bytes.len() < 254 {
            array[..bytes.len()].copy_from_slice(bytes);
        }
        Self(array)
    }
}

impl Pi for Pi99 {
    fn parse(data: &[u8]) -> nom::IResult<&[u8], Self>
    where
        Self: Sized,
    {
        let (data, inner) = nom::bytes::complete::take(254usize)(data)?;
        Ok((data, Pi99(inner.try_into().unwrap_or([0; 254]))))
    }
}

impl PiAsBytes for Pi99 {
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
    fn parse_from_str() {
        let s = "test";
        let pi99 = Pi99::from_str(s);
        let bytes = s.as_bytes();
        assert_eq!(pi99.0, [bytes, &[0u8; 250]].concat().as_slice());
    }

    #[test]
    fn test_parse_pi99() {
        let default = Pi99::default();
        assert_eq!(default.as_bytes(), [0; 254]);
        let pi = Pi99::parse(&[1; 254]).unwrap().1;
        assert_eq!(pi.as_bytes(), [1; 254]);
    }
}
