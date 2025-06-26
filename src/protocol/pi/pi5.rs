use nom::sequence::tuple;
use nom::IResult;

use crate::protocol::pi::Pi;

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
        let (data, new_password) = if data.len() > 8 {
            let (data, new_password) = nom::bytes::complete::take(8usize)(data)?;
            (data, Some(new_password))
        } else {
            (data, None)
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
