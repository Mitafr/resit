use nom::error::Error;

use crate::{
    error::PesitError,
    protocol::pi::{Pi, PiDefinition, PiOption},
};

pub(crate) mod connect;
pub(crate) mod create;
pub(crate) mod prelude;
pub(crate) mod release;

pub(crate) fn parse_pi<P: Pi>(input: &[u8]) -> Result<(&[u8], P), PesitError> {
    let (input, pi_code) =
        nom::bytes::complete::take::<usize, &[u8], Error<&[u8]>>(1usize)(input).unwrap();
    log::debug!("Parsing Pi with code {}", pi_code[0]);
    if P::optional() == PiOption::Mandatory && pi_code[0] != P::code() {
        return Err(PesitError::Parse(format!(
            "Pis of the frame are unordered, excepted {} but got {}",
            P::code(),
            pi_code[0]
        )));
    }
    match P::parse(input) {
        Ok(t) => Ok(t),
        Err(e) => Err(PesitError::Parse(e.to_string())),
    }
}
