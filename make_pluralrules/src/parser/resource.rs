// one public function that
// takes a string from the file
// and returns a structure with rules per locale

extern crate serde;
extern crate serde_json;

use std::collections::HashMap;
use std::error::Error;
// use std::fs::File;
// use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Resource {
    pub supplemental: Supplemental,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Version {
    #[serde(rename = "_number")]
    pub number: String,
    #[serde(rename = "_unicodeVersion")]
    pub unicode_version: String,
    #[serde(rename = "_cldrVersion")]
    pub cldr_version: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Supplemental {
    pub version: Version,
    #[serde(rename = "plurals-type-cardinal")]
    pub plurals_type_cardinal: Option<HashMap<String, HashMap<String, String>>>,
    #[serde(rename = "plurals-type-ordinal")]
    pub plurals_type_ordinal: Option<HashMap<String, HashMap<String, String>>>,
}

/// Will parse a CLDR compliant source from a &str.
pub fn parse_plurals_resource_from_string(body: &str) -> Result<Resource, Box<Error>> {
    let u = serde_json::from_str(body)?;
    Ok(u)
}

// Will parser a CLDR compliant source from a local file.
// pub fn parse_plurals_resource_from_file<P: AsRef<Path>>(path: P) -> Result<Resource, Box<Error>> {
//     let file = File::open(path)?;
//     let u = serde_json::from_reader(file)?;
//     Ok(u)
// }
