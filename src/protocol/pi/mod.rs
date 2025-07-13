use nom::IResult;

pub(crate) mod pi1;
pub(crate) mod pi11;
pub(crate) mod pi12;
pub(crate) mod pi13;
pub(crate) mod pi14;
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

use prelude::*;

/// This enum defines the types of protocol identifiers (PIs) used in the PESIT protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PiType {
    /// String
    C,
    /// Numeric
    N,
    /// Symbolic
    S,
    /// Bitmask
    M,
    /// Datetime
    D,
    /// Aggregate
    A,
}

/// This trait defines the parsing behavior for all protocol identifier structures.
pub trait Pi: Default + PiDefinition {
    /// Parses the given data into a protocol identifier structure.
    fn parse(data: &[u8]) -> IResult<&[u8], Self>
    where
        Self: Sized;

    /// Converts the protocol identifier structure into a byte vector.
    fn as_bytes(&self) -> Vec<u8>;
}

/// This trait defines the definition of a protocol identifier
pub trait PiDefinition: Default {
    /// Returns the type of the protocol identifier.
    fn ptype(&self) -> PiType;

    /// Returns the length of the protocol identifier.
    fn len(&self) -> usize;

    /// Returns the code of the protocol identifier.
    fn code(&self) -> u8;

    /// Returns the alias name of the protocol identifier.
    fn alias(&self) -> &'static str;
}

macro_rules! declare_pi_definition {
    ($p: ident, $code: tt, $len: tt, $ptype: path, $name: tt) => {
        impl PiDefinition for $p {
            fn ptype(&self) -> PiType {
                $ptype
            }
            fn len(&self) -> usize {
                $len
            }
            fn code(&self) -> u8 {
                $code
            }
            fn alias(&self) -> &'static str {
                $name
            }
        }
    };
}

declare_pi_definition!(Pi1, 1, 1, PiType::S, "CRC Usage");
declare_pi_definition!(Pi2, 2, 3, PiType::A, "Diagnostics");
declare_pi_definition!(Pi3, 3, 24, PiType::C, "Caller Identification");
declare_pi_definition!(Pi4, 4, 24, PiType::C, "Server Identification");
declare_pi_definition!(Pi5, 5, 16, PiType::C, "Access Control");
declare_pi_definition!(Pi6, 6, 1, PiType::N, "Version Number");
declare_pi_definition!(Pi7, 7, 3, PiType::A, "Option : Checkpointing");
declare_pi_definition!(Pi11, 11, 2, PiType::N, "File Type");
declare_pi_definition!(Pi12, 12, 76, PiType::C, "File Name");
declare_pi_definition!(Pi13, 13, 3, PiType::N, "Transfer Identifier");
declare_pi_definition!(Pi14, 14, 1, PiType::M, "Requested Attributes");
declare_pi_definition!(Pi15, 15, 1, PiType::S, "Recovered Transfer");
declare_pi_definition!(Pi16, 16, 1, PiType::S, "Data Coding");
declare_pi_definition!(Pi17, 17, 1, PiType::S, "Transfer Priority");
declare_pi_definition!(Pi25, 25, 2, PiType::N, "Maximum Side Of A Data Element");
declare_pi_definition!(Pi31, 31, 1, PiType::M, "Article Format");
declare_pi_definition!(Pi32, 32, 2, PiType::N, "Article Length");
declare_pi_definition!(Pi33, 33, 1, PiType::S, "File Attributes");
declare_pi_definition!(Pi34, 34, 2, PiType::N, "Use Of The Signature");
declare_pi_definition!(Pi36, 36, 64, PiType::N, "SIT Mac");
declare_pi_definition!(Pi37, 37, 80, PiType::C, "File Label");
declare_pi_definition!(Pi41, 41, 1, PiType::S, "Storage Reservation Unit");
declare_pi_definition!(Pi42, 42, 4, PiType::N, "Maximum Reserved Space");
declare_pi_definition!(Pi51, 51, 12, PiType::D, "Date And Time Of Creation");
declare_pi_definition!(Pi52, 52, 12, PiType::D, "Date And Time Of Last Access");
declare_pi_definition!(Pi61, 61, 24, PiType::C, "Customer Identifier");
declare_pi_definition!(Pi62, 62, 24, PiType::C, "Bank Identifier");
declare_pi_definition!(Pi71, 71, 3, PiType::A, "Authentication Type");
declare_pi_definition!(Pi99, 99, 254, PiType::N, "Free Text");

#[cfg(test)]
mod tests_definition {
    use super::*;

    #[test]
    fn pi_def() {
        let pi1 = Pi1(true);
        assert_eq!(pi1.code(), 1);
        assert_eq!(pi1.len(), 1);
        assert_eq!(pi1.ptype(), PiType::S);
        assert_eq!(pi1.alias(), "CRC Usage")
    }
}
