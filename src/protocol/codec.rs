use super::frame::Frame;
use crate::{error::PesitError, protocol::parser::Parser};
use bytes::{Buf, BufMut, BytesMut};
use tokio_util::codec::{Decoder, Encoder};

#[derive(Debug)]
pub(crate) struct PesitCodec;

impl Decoder for PesitCodec {
    type Item = Frame<Vec<u8>>;
    type Error = PesitError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < 5 {
            return Ok(None);
        }

        let frame = Parser::parse(src);
        src.advance(frame.len);
        Ok(Some(frame))
    }
}

impl Encoder<Frame<Vec<u8>>> for PesitCodec {
    type Error = PesitError;

    fn encode(&mut self, item: Frame<Vec<u8>>, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let header = item.header;
        dst.put_u16(item.header.length);
        let kind = header.kind.into_arr();
        dst.put_u8(kind[0]);
        dst.put_u8(kind[1]);
        dst.put_u8(header.dest_id);
        dst.put_u8(header.oct6);
        dst.put_slice(&item.payload);
        Ok(())
    }
}
