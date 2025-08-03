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
pub(crate) mod pi22;
pub(crate) mod pi23;
pub(crate) mod pi25;
pub(crate) mod pi3;
pub(crate) mod pi31;
pub(crate) mod pi32;
pub(crate) mod pi33;
pub(crate) mod pi34;
pub(crate) mod pi36;
pub(crate) mod pi37;
pub(crate) mod pi38;
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
pub trait Pi: Default + PiDefinition + PiAsBytes {
    /// Parses the given data into a protocol identifier structure.
    fn parse(data: &[u8]) -> IResult<&[u8], Self>
    where
        Self: Sized;
}

pub trait PiAsBytes {
    /// Converts the protocol identifier structure into a byte vector.
    fn as_bytes(&self) -> Vec<u8>;
}

/// This trait defines the definition of a protocol identifier
pub trait PiDefinition: Default {
    /// Returns the type of the protocol identifier.
    fn ptype() -> PiType;

    /// Returns the length of the protocol identifier.
    fn len() -> usize;

    /// Returns the code of the protocol identifier.
    fn code() -> u8;

    /// Returns the alias name of the protocol identifier.
    fn alias() -> &'static str;

    /// Returns true if the Pi is Optional, false otherwise.
    fn optional() -> PiOption;
}

/// This macro is used to declare a new protocol identifier structure.
/// It defines the type, length, code, and alias name of the protocol identifier.
macro_rules! declare_pi_definition {
    ($p: ident, $code: tt, $len: tt, $ptype: path, $name: tt, $optional: path) => {
        impl PiDefinition for $p {
            fn ptype() -> PiType {
                $ptype
            }
            fn len() -> usize {
                $len
            }
            fn code() -> u8 {
                $code
            }
            fn alias() -> &'static str {
                $name
            }
            fn optional() -> PiOption {
                $optional
            }
        }
    };
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PiOption {
    #[default]
    Mandatory,
    Optional,
    WithDefault,
}

declare_pi_definition!(Pi1, 1, 1, PiType::S, "CRC Usage", PiOption::WithDefault);
declare_pi_definition!(Pi2, 2, 3, PiType::A, "Diagnostics", PiOption::Mandatory);
declare_pi_definition!(
    Pi3,
    3,
    24,
    PiType::C,
    "Caller Identification",
    PiOption::Optional
);
declare_pi_definition!(
    Pi4,
    4,
    24,
    PiType::C,
    "Server Identification",
    PiOption::Optional
);
declare_pi_definition!(Pi5, 5, 16, PiType::C, "Access Control", PiOption::Mandatory);
declare_pi_definition!(Pi6, 6, 1, PiType::N, "Version Number", PiOption::Mandatory);
declare_pi_definition!(
    Pi7,
    7,
    3,
    PiType::A,
    "Option : Checkpointing",
    PiOption::WithDefault
);
declare_pi_definition!(Pi11, 11, 2, PiType::N, "File Type", PiOption::Mandatory);
declare_pi_definition!(Pi12, 12, 76, PiType::C, "File Name", PiOption::Mandatory);
declare_pi_definition!(
    Pi13,
    13,
    3,
    PiType::N,
    "Transfer Identifier",
    PiOption::Mandatory
);
declare_pi_definition!(
    Pi14,
    14,
    1,
    PiType::M,
    "Requested Attributes",
    PiOption::WithDefault
);
declare_pi_definition!(
    Pi15,
    15,
    1,
    PiType::S,
    "Recovered Transfer",
    PiOption::WithDefault
);
declare_pi_definition!(Pi16, 16, 1, PiType::S, "Data Coding", PiOption::WithDefault);
declare_pi_definition!(
    Pi17,
    17,
    1,
    PiType::S,
    "Transfer Priority",
    PiOption::Mandatory
);
declare_pi_definition!(Pi22, 22, 1, PiType::S, "Access Type", PiOption::Mandatory);
declare_pi_definition!(Pi23, 23, 1, PiType::S, "Resync", PiOption::WithDefault);
declare_pi_definition!(
    Pi25,
    25,
    2,
    PiType::N,
    "Maximum Side Of A Data Element",
    PiOption::Mandatory
);
declare_pi_definition!(
    Pi31,
    31,
    1,
    PiType::M,
    "Article Format",
    PiOption::WithDefault
);
declare_pi_definition!(
    Pi32,
    32,
    2,
    PiType::N,
    "Article Length",
    PiOption::Mandatory
);
declare_pi_definition!(
    Pi33,
    33,
    1,
    PiType::S,
    "File Attributes",
    PiOption::WithDefault
);
declare_pi_definition!(
    Pi34,
    34,
    2,
    PiType::N,
    "Use Of The Signature",
    PiOption::WithDefault
);
declare_pi_definition!(Pi36, 36, 64, PiType::N, "SIT Mac", PiOption::Optional);
declare_pi_definition!(Pi37, 37, 80, PiType::C, "File Label", PiOption::Optional);
declare_pi_definition!(Pi38, 38, 2, PiType::N, "Key Length", PiOption::Optional);
declare_pi_definition!(
    Pi41,
    41,
    1,
    PiType::S,
    "Storage Reservation Unit",
    PiOption::WithDefault
);
declare_pi_definition!(
    Pi42,
    42,
    4,
    PiType::N,
    "Maximum Reserved Space",
    PiOption::Mandatory
);
declare_pi_definition!(
    Pi51,
    51,
    12,
    PiType::D,
    "Date And Time Of Creation",
    PiOption::Mandatory
);
declare_pi_definition!(
    Pi52,
    52,
    12,
    PiType::D,
    "Date And Time Of Last Access",
    PiOption::Optional
);
declare_pi_definition!(
    Pi61,
    61,
    24,
    PiType::C,
    "Customer Identifier",
    PiOption::Mandatory
);
declare_pi_definition!(
    Pi62,
    62,
    24,
    PiType::C,
    "Bank Identifier",
    PiOption::Mandatory
);
declare_pi_definition!(
    Pi71,
    71,
    3,
    PiType::A,
    "Authentication Type",
    PiOption::WithDefault
);
// declare_pi_definition!(Pi91, 91, 4096, PiType::N, "Message", PiOption::Optional);
declare_pi_definition!(Pi99, 99, 254, PiType::N, "Free Text", PiOption::Optional);

#[cfg(test)]
mod tests_definition {
    use super::*;

    #[test]
    fn pi_def() {
        assert_eq!(Pi1::code(), 1);
        assert_eq!(Pi1::len(), 1);
        assert_eq!(Pi1::ptype(), PiType::S);
        assert_eq!(Pi1::alias(), "CRC Usage");
    }
}
