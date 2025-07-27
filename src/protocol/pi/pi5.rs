use nom::IResult;

use crate::protocol::pi::{Pi, PiAsBytes, PiDefinition};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Pi5 {
    password: [u8; 8],
    new_password: Option<[u8; 8]>,
}

impl Pi for Pi5 {
    fn parse(data: &[u8]) -> IResult<&[u8], Self>
    where
        Self: Sized,
    {
        let (data, password) = nom::bytes::complete::take(8usize)(data)?;
        let (data, new_password) = if data.is_empty() {
            (data, None)
        } else {
            let (data, new_password) = nom::bytes::complete::take(8usize)(data)?;
            (data, Some(new_password))
        };

        Ok((
            data,
            Pi5 {
                password: password.try_into().unwrap_or_default(),
                new_password: new_password.map(|np| np.try_into().unwrap_or_default()),
            },
        ))
    }
}

impl PiAsBytes for Pi5 {
    fn as_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(17);
        buf.push(Self::code());
        buf.extend_from_slice(&self.password);
        if let Some(new_password) = &self.new_password {
            buf.extend_from_slice(new_password);
        } else {
            buf.extend_from_slice(&[0; 8]); // Fill with zeros if no new password
        }
        buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pi5_parse() {
        let data = "test1234".as_bytes();
        let (remain, pi5) = Pi5::parse(data).unwrap();
        assert_eq!(pi5.password, *data);
        assert!(remain.is_empty());
    }

    #[test]
    fn test_pi5_parse_with_new_password() {
        let data = "test1234new12345".as_bytes();
        let (remain, pi5) = Pi5::parse(data).unwrap();
        assert_eq!(
            pi5.password,
            [b't', b'e', b's', b't', b'1', b'2', b'3', b'4']
        );
        assert_eq!(
            pi5.new_password,
            Some([b'n', b'e', b'w', b'1', b'2', b'3', b'4', b'5'])
        );
        assert!(remain.is_empty());
    }

    #[test]
    fn test_as_bytes() {
        let data = "test1234".as_bytes();
        let (_, pi5) = Pi5::parse(data).unwrap();
        let mut excepted = vec![5];
        excepted.extend(data);
        excepted.extend([0u8; 8]);
        assert_eq!(excepted, pi5.as_bytes());
    }
}
