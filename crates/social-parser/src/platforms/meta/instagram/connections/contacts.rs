use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::BufReader,
    path::{absolute, Path},
};

use crate::{common::ParseError, platforms::meta::instagram::StringData};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Contacts {
    pub synced_contacts: Option<SyncedContacts>,
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

            match path.file_name().and_then(|s| s.to_str()) {
                Some("synced_contacts.json") => {
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(deny_unknown_fields)]
pub struct SyncedContacts {
    pub contacts_contact_info: Vec<ContactInfo>,
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

        serde_json::from_reader(reader).map_err(|e| ParseError::Serde(path.to_owned(), e))
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
#[serde(deny_unknown_fields)]
pub struct ContactInfo {
    pub title: String,
    pub media_map_data: MediaMapData,
    pub string_map_data: ContactStringMapData,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
#[serde(deny_unknown_fields)]
pub struct MediaMapData {}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
#[serde(deny_unknown_fields)]
pub struct ContactStringMapData {
    #[serde(rename = "First Name")]
    pub first_name: StringData,
    #[serde(rename = "Last Name")]
    pub last_name: StringData,
    #[serde(rename = "Contact Information")]
    pub contact_inform: StringData,
    #[serde(rename = "Imported Time")]
    pub imported_time: StringData,
}
