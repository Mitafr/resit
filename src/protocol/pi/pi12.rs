use nom::IResult;

use crate::protocol::pi::{Pi, PiAsBytes};

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub(crate) enum VersionRequestType {
    #[default]
    NamedReference = 0,
    LatestVersion = 1,
    MissingVersions = 2,
    AllVersions = 3,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub(crate) enum IdentifierType {
    #[default]
    MutuallyAggreed = 0,
    Standard = 1,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, bon::Builder)]
pub struct Pi12 {
    identifier: IdentifierType,
    reference_type: Option<VersionRequestType>,
    file_reference: [u8; 12],
}

impl Pi12 {
    fn parse_identifier(data: Option<&u8>) -> IdentifierType {
        match data {
            Some(0) => IdentifierType::MutuallyAggreed,
            Some(1) => IdentifierType::Standard,
            _ => IdentifierType::default(),
        }
    }

    fn parse_version(data: Option<&u8>) -> VersionRequestType {
        match data {
            Some(0) => VersionRequestType::NamedReference,
            Some(1) => VersionRequestType::LatestVersion,
            Some(2) => VersionRequestType::MissingVersions,
            Some(3) => VersionRequestType::AllVersions,
            _ => VersionRequestType::default(),
        }
    }
}

impl Pi for Pi12 {
    fn parse(data: &[u8]) -> IResult<&[u8], Self>
    where
        Self: Sized,
    {
        if data.len() <= 14 {
            return Err(nom::Err::Incomplete(nom::Needed::new(14)));
        }
        let identifier = Self::parse_identifier(data.first());
        match identifier {
            IdentifierType::MutuallyAggreed => {
                let mut file_reference = [0u8; 12];
                file_reference.copy_from_slice(&data[2..14]);
                Ok((
                    &data[14..],
                    Pi12 {
                        identifier,
                        reference_type: Some(VersionRequestType::NamedReference),
                        file_reference,
                    },
                ))
            }
            IdentifierType::Standard => {
                let reference_type = Self::parse_version(data.get(1));
                let mut file_reference = [0u8; 12];
                file_reference[..6].copy_from_slice(&data[2..8]);
                file_reference[6..].copy_from_slice(&data[8..14]);
                Ok((
                    &data[14..],
                    Pi12 {
                        identifier,
                        reference_type: Some(reference_type),
                        file_reference,
                    },
                ))
            }
        }
    }
}

impl PiAsBytes for Pi12 {
    fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.push(self.identifier as u8);
        match self.reference_type {
            Some(ref ref_type) => bytes.push(*ref_type as u8),
            None => bytes.push(0),
        }
        bytes.extend_from_slice(&self.file_reference);
        bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_mutually_aggreed() {
        let mut data = vec![0, 0];
        data.extend(1u8..=13u8);
        let (rem, pi) = Pi12::parse(&data).unwrap();
        assert_eq!(rem, &data[14..]);
        assert_eq!(pi.identifier, IdentifierType::MutuallyAggreed);
        assert_eq!(pi.reference_type, Some(VersionRequestType::NamedReference));
        assert_eq!(pi.file_reference, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
    }

    #[test]
    fn test_parse_standard_named_reference() {
        let mut data = vec![1, 0];
        data.extend(2u8..=8u8);
        data.extend(9u8..=14u8);
        let (rem, pi) = Pi12::parse(&data).unwrap();
        assert_eq!(rem, &data[14..]);
        assert_eq!(pi.identifier, IdentifierType::Standard);
        assert_eq!(pi.reference_type, Some(VersionRequestType::NamedReference));
        assert_eq!(pi.file_reference, [2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]);
    }

    #[test]
    fn test_parse_standard_latest_version() {
        let mut data = vec![1, 1];
        data.extend(2u8..=8u8);
        data.extend(9u8..=14u8);
        let (rem, pi) = Pi12::parse(&data).unwrap();
        assert_eq!(rem, &data[14..]);
        assert_eq!(pi.identifier, IdentifierType::Standard);
        assert_eq!(pi.reference_type, Some(VersionRequestType::LatestVersion));
        assert_eq!(pi.file_reference, [2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]);
    }

    #[test]
    fn test_parse_standard_missing_versions() {
        let mut data = vec![1, 2];
        data.extend(2u8..=8u8);
        data.extend(9u8..=14u8);
        let (rem, pi) = Pi12::parse(&data).unwrap();
        assert_eq!(rem, &data[14..]);
        assert_eq!(pi.identifier, IdentifierType::Standard);
        assert_eq!(pi.reference_type, Some(VersionRequestType::MissingVersions));
        assert_eq!(pi.file_reference, [2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]);
    }

    #[test]
    fn test_parse_standard_all_versions() {
        let mut data = vec![1, 3];
        data.extend(2u8..=8u8);
        data.extend(9u8..=14u8);
        let (rem, pi) = Pi12::parse(&data).unwrap();
        assert_eq!(rem, &data[14..]);
        assert_eq!(pi.identifier, IdentifierType::Standard);
        assert_eq!(pi.reference_type, Some(VersionRequestType::AllVersions));
        assert_eq!(pi.file_reference, [2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]);
    }

    #[test]
    fn test_parse_incomplete() {
        let data = vec![0, 0, 1, 2, 3];
        let result = Pi12::parse(&data);
        assert!(
            result.is_err(),
            "Should not panic, but may return wrong result due to unchecked slice"
        );
    }
}
