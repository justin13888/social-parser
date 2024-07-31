use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::BufReader,
    path::{absolute, Path},
};

use crate::common::ParseError;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Contacts {
    synced_contacts: Option<SyncedContacts>,
} // TODO: Complete

impl TryFrom<&Path> for Contacts {
    type Error = ParseError;

    /// Load from a file. Assumes path is a directory.
    fn try_from(path: &Path) -> Result<Contacts, Self::Error> {
        if !path.is_dir() {
            return Err(ParseError::UnexpectedFormat(format!(
                "Found unexpected non-directory in Contacts: {:?}",
                path
            )));
        }

        let mut synced_contacts = None;

        for entry in path.read_dir()? {
            let entry = entry?;
            let path = entry.path();

            // Must be a file
            if !path.is_file() {
                return Err(ParseError::UnexpectedFormat(format!(
                    "Found unexpected non-file in Contacts: {:?}",
                    absolute(path)
                )));
            }

            match path.file_stem().and_then(|s| s.to_str()) {
                Some("synced_contacts") => {
                    synced_contacts = Some(SyncedContacts::try_from(path.as_path())?);
                }
                _ => {
                    return Err(ParseError::UnexpectedFormat(format!(
                        "Found unexpected file in Contacts: {:?}",
                        absolute(path)
                    )));
                }
            }
        }

        Ok(Contacts { synced_contacts })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SyncedContacts {
    contacts_contact_info: Vec<ContactInfo>,
}

impl TryFrom<&Path> for SyncedContacts {
    type Error = ParseError;

    /// Load from a file. Assumes path is a file.
    fn try_from(path: &Path) -> Result<SyncedContacts, Self::Error> {
        if !path.is_file() {
            return Err(ParseError::UnexpectedFormat(format!(
                "Found unexpected non-file in SyncedContacts: {:?}",
                path
            )));
        }

        let file = File::open(path)?;
        let reader = BufReader::new(file);

        serde_json::from_reader(reader).map_err(ParseError::Serde)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ContactInfo {
    title: String,
    media_map_data: MediaMapData,
    string_map_data: StringMapData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct MediaMapData {}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct StringMapData {
    #[serde(rename = "First Name")]
    first_name: StringMap,
    #[serde(rename = "Last Name")]
    last_name: StringMap,
    #[serde(rename = "Contact Information")]
    contact_inform: StringMap,
    #[serde(rename = "Imported Time")]
    imported_time: StringMap,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct StringMap {
    href: String,
    value: String,
    timestamp: i32,
}
