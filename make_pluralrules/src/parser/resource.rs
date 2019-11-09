//! This reference parser uses serde_json to produce the plural rules from a CLDR data JSON file.

use serde::{Serialize, Deserialize};

use std::collections::BTreeMap;
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct Resource {
    pub supplemental: Supplemental,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Version {
    #[serde(rename = "_number")]
    pub number: Option<String>,
    #[serde(rename = "_unicodeVersion")]
    pub unicode_version: String,
    #[serde(rename = "_cldrVersion")]
    pub cldr_version: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Supplemental {
    pub version: Version,
    #[serde(rename = "plurals-type-cardinal")]
    pub plurals_type_cardinal: Option<BTreeMap<String, BTreeMap<String, String>>>,
    #[serde(rename = "plurals-type-ordinal")]
    pub plurals_type_ordinal: Option<BTreeMap<String, BTreeMap<String, String>>>,
}

/// Will parse a CLDR compliant source from a &str.
pub fn parse_plurals_resource_from_string(body: &str) -> Result<Resource, Box<dyn Error>> {
    let u = serde_json::from_str(body)?;
    Ok(u)
}
