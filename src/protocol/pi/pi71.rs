use nom::IResult;

use crate::protocol::pi::Pi;

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
        if let Ok((data, auth)) =
            nom::bits::complete::bool::<_, nom::error::Error<(&[u8], usize)>>((data, 1usize))
        {
            pi.auth = auth;
            pi.auth_type =
                nom::bits::complete::take::<_, u8, _, nom::error::Error<(&[u8], usize)>>(1usize)(
                    data,
                )
                .map(|(_, algo)| match algo {
                    0 => Some(AuthAlgo::Rsa),
                    1 => Some(AuthAlgo::Des),
                    _ => None,
                })
                .ok()
                .flatten();
            pi.auth_procedure =
                nom::bits::complete::take::<_, u8, _, nom::error::Error<(&[u8], usize)>>(1usize)(
                    data,
                )
                .map(|(_data, proc_type)| match proc_type {
                    0 => Some(AuthProcedure::CertExchange),
                    1 => Some(AuthProcedure::ThreeWayAuth),
                    2 => Some(AuthProcedure::ThreeWayAuthDesOnly),
                    _ => None,
                })
                .ok()
                .flatten();
        }
        Ok((data, pi))
    }

    fn as_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(3);
        buf.push(self.auth as u8);
        if self.auth {
            buf.push(self.auth_type.unwrap() as u8);
            buf.push(self.auth_procedure.unwrap() as u8);
        }
        buf
    }

    fn code(&self) -> u8 {
        71
    }

    fn len(&self) -> usize {
        3
    }

    fn ptype(&self) -> super::PiType {
        super::PiType::A
    }
}
