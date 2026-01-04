use std::str::FromStr;
use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
}

#[derive(Debug)]
pub enum VersionParseError {
    InvalidFormat,
    InvalidNumber,
}

impl FromStr for Version {
    type Err = VersionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('.').collect();

        match parts.as_slice() {
            [major, minor] => Ok(Version {
                major: major.parse().map_err(|_| VersionParseError::InvalidNumber)?,
                minor: minor.parse().map_err(|_| VersionParseError::InvalidNumber)?,
                patch: 0,
            }),
            [major, minor, patch] => Ok(Version {
                major: major.parse().map_err(|_| VersionParseError::InvalidNumber)?,
                minor: minor.parse().map_err(|_| VersionParseError::InvalidNumber)?,
                patch: patch.parse().map_err(|_| VersionParseError::InvalidNumber)?,
            }),
            _ => Err(VersionParseError::InvalidFormat),
        }
    }
}
impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.patch == 0 {
            write!(f, "{}.{}", self.major, self.minor)
        } else {
            write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
        }
    }
}

impl fmt::Display for VersionParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VersionParseError::InvalidFormat => {
                write!(f, "invalid version format (expected MAJOR.MINOR or MAJOR.MINOR.PATCH)")
            }
            VersionParseError::InvalidNumber => {
                write!(f, "invalid number in version (expected numeric components)")
            }
        }
    }
}
