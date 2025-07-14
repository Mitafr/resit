use nom::IResult;

use crate::protocol::pi::{Pi, PiAsBytes};

#[derive(Debug, Clone, Copy, PartialEq)]
enum AuthAlgo {
    Rsa,
    Des,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum AuthProcedure {
    CertExchange,
    ThreeWayAuth,
    ThreeWayAuthDesOnly,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Pi71 {
    auth: bool,
    auth_type: Option<AuthAlgo>,
    auth_procedure: Option<AuthProcedure>,
}

impl Pi for Pi71 {
    fn parse(data: &[u8]) -> IResult<&[u8], Self> {
        let mut pi = Pi71 {
            auth: false,
            auth_type: None,
            auth_procedure: None,
        };
        let (data, auth) = nom::bytes::complete::take(1usize)(data)?;
        pi.auth = auth[0] == 1;
        if !pi.auth {
            return Ok((data, pi));
        }
        let (data, auth_type) = nom::bytes::complete::take(1usize)(data)?;
        pi.auth_type = match auth_type[0] {
            0 => Some(AuthAlgo::Rsa),
            1 => Some(AuthAlgo::Des),
            _ => unreachable!(),
        };
        let (data, auth_procedure) = nom::bytes::complete::take(1usize)(data)?;
        pi.auth_procedure = match auth_procedure[0] {
            0 => Some(AuthProcedure::CertExchange),
            1 => Some(AuthProcedure::ThreeWayAuth),
            2 => Some(AuthProcedure::ThreeWayAuthDesOnly),
            _ => unreachable!(),
        };
        Ok((data, pi))
    }
}

impl PiAsBytes for Pi71 {
    fn as_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(3);
        buf.push(u8::from(self.auth));
        if self.auth {
            buf.push(self.auth_type.unwrap() as u8);
            buf.push(self.auth_procedure.unwrap() as u8);
        }
        buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_no_auth() {
        let pi = Pi71::parse(&[0, 0, 0]).unwrap().1;
        assert!(!pi.auth);
        assert_eq!(pi.auth_type, None);
        assert_eq!(pi.auth_procedure, None);
        assert_eq!(pi.as_bytes(), [0]);
    }

    #[test]
    fn test_parse_pi71() {
        let pi = Pi71::parse(&[1, 0, 0]).unwrap().1;
        assert!(pi.auth);
        assert_eq!(pi.auth_type, Some(AuthAlgo::Rsa));
        assert_eq!(pi.auth_procedure, Some(AuthProcedure::CertExchange));
        assert_eq!(pi.as_bytes(), [1, 0, 0]);
        let pi = Pi71::parse(&[1, 1, 0]).unwrap().1;
        assert!(pi.auth);
        assert_eq!(pi.auth_type, Some(AuthAlgo::Des));
        assert_eq!(pi.auth_procedure, Some(AuthProcedure::CertExchange));
    }
}
