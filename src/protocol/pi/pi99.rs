use crate::protocol::pi::Pi;

pub struct Pi99(pub [u8; 254]);

impl Pi for Pi99 {
    fn parse(data: &[u8]) -> nom::IResult<&[u8], Self>
    where
        Self: Sized,
    {
        let (data, inner) = nom::bytes::complete::take(254usize)(data)?;
        Ok((data, Pi99(inner.try_into().unwrap_or([0; 254]))))
    }
}
