use nom::IResult;

pub(crate) mod pi1;
pub(crate) mod pi11;
pub(crate) mod pi12;
pub(crate) mod pi13;
pub(crate) mod pi15;
pub(crate) mod pi16;
pub(crate) mod pi17;
pub(crate) mod pi2;
pub(crate) mod pi25;
pub(crate) mod pi3;
pub(crate) mod pi31;
pub(crate) mod pi32;
pub(crate) mod pi33;
pub(crate) mod pi34;
pub(crate) mod pi36;
pub(crate) mod pi37;
pub(crate) mod pi4;
pub(crate) mod pi41;
pub(crate) mod pi42;
pub(crate) mod pi5;
pub(crate) mod pi51;
pub(crate) mod pi52;
pub(crate) mod pi6;
pub(crate) mod pi61;
pub(crate) mod pi62;
pub(crate) mod pi7;
pub(crate) mod pi71;
pub(crate) mod pi91;
pub(crate) mod pi99;
pub mod prelude;

/// This trait defines the parsing behavior for all protocol identifier structures.
pub trait Pi: Default {
    /// Parses the given data into a protocol identifier structure.
    fn parse(data: &[u8]) -> IResult<&[u8], Self>
    where
        Self: Sized;
}
