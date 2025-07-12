use nom::IResult;

use crate::protocol::pi::Pi;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Pi7 {
    sync_interval: u16,
    window: u8,
}

impl Pi for Pi7 {
    fn parse(data: &[u8]) -> IResult<&[u8], Self>
    where
        Self: Sized,
    {
        let (data, sync_interval) =
            nom::number::complete::u16(nom::number::Endianness::Native)(data)?;
        if sync_interval > 0 {
            let (data, window) = nom::number::complete::u8(data)?;
            Ok((
                data,
                Pi7 {
                    sync_interval,
                    window,
                },
            ))
        } else {
            Ok((
                data,
                Pi7 {
                    sync_interval,
                    window: 0,
                },
            ))
        }
    }

    fn as_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(3);
        buf.extend_from_slice(&self.sync_interval.to_be_bytes());
        buf.push(self.window);
        buf
    }

    fn code(&self) -> u8 {
        7
    }

    fn len(&self) -> usize {
        3
    }

    fn ptype(&self) -> super::PiType {
        super::PiType::A
    }
}
