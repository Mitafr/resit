use crate::{error::PesitError, protocol::pi::Pi};

pub(crate) mod connect;
pub(crate) mod create;
pub(crate) mod prelude;
pub(crate) mod release;

pub(crate) fn parse_pi<P: Pi>(input: &[u8]) -> Result<(&[u8], P), PesitError> {
    match P::parse(input) {
        Ok(t) => Ok(t),
        Err(e) => Err(PesitError::Parse(e.to_string())),
    }
}
