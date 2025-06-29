use nom::IResult;

use crate::protocol::pi::Pi;

#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) enum VersionRequestType {
    #[default]
    NamedReference = 0,
    LatestVersion = 1,
    MissingVersions = 2,
    AllVersions = 3,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) enum IdentifierType {
    #[default]
    MutuallyAggreed = 0,
    Standard = 1,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Pi12 {
    identifier: IdentifierType,
    reference_type: Option<VersionRequestType>,
    file_reference: Option<[u8; 12]>,
}

impl Pi for Pi12 {
    fn parse(data: &[u8]) -> IResult<&[u8], Self>
    where
        Self: Sized,
    {
        let identifier = match data.first() {
            Some(0) => IdentifierType::MutuallyAggreed,
            Some(1) => IdentifierType::Standard,
            _ => IdentifierType::default(),
        };
        match identifier {
            IdentifierType::MutuallyAggreed => {
                let mut file_reference = [0u8; 12];
                file_reference.copy_from_slice(&data[2..14]);
                Ok((
                    &data[13..],
                    Pi12 {
                        identifier,
                        reference_type: Some(VersionRequestType::NamedReference),
                        file_reference: Some(file_reference),
                    },
                ))
            }
            IdentifierType::Standard => {
                let reference_type = match data.get(2) {
                    Some(0) => VersionRequestType::NamedReference,
                    Some(1) => VersionRequestType::LatestVersion,
                    Some(2) => VersionRequestType::MissingVersions,
                    Some(3) => VersionRequestType::AllVersions,
                    _ => VersionRequestType::default(),
                };
                let mut file_reference = [0u8; 12];
                file_reference[..6].copy_from_slice(&data[2..8]);
                file_reference[6..].copy_from_slice(&data[8..14]);
                Ok((
                    &data[1..],
                    Pi12 {
                        identifier,
                        reference_type: Some(reference_type),
                        file_reference: Some(file_reference),
                    },
                ))
            }
        }
    }
}
