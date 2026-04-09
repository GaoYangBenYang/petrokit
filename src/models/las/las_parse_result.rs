#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use crate::models::las::version_information::VersionInformation;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LasParseResult {
    pub version: VersionInformation,
    pub well: LasData,
    pub curve: LasData,
    pub parameter: LasData,
    pub other: LasData,
    pub ascii: LasData,
}
