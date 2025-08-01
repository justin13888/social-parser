pub mod activity;
pub mod connections;
pub mod media;

use std::path::{absolute, Path};

use activity::Activity;
use connections::Connections;
use media::Media;
use serde::{Deserialize, Serialize};

use crate::common::{ParseError, WriteError};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct InstagramArchive {
    pub connections: Option<Connections>,
    pub media: Option<Media>,
    pub activity: Option<Activity>,
}

impl InstagramArchive {
    /// Load from a folder
    /// Assumes path is a directory.
    fn from_folder<P: AsRef<Path>>(path: P) -> Result<Self, ParseError> {
        assert!(path.as_ref().is_dir());

        // Load all directories in the directory
        let mut connections = None;
        let mut media = None;
        let mut activity = None;

        for entry in path.as_ref().read_dir()? {
            let entry = entry?;
            let path = entry.path();

            // Must be a directory
            if !path.is_dir() {
                return Err(ParseError::UnexpectedFormat(format!(
                    "Found unexpected non-directory in Instagram archive: {:?}",
                    absolute(path)
                )));
            }

            match path.file_stem().and_then(|s| s.to_str()) {
                Some("ads_information") => {}
                Some("apps_and_websites_off_of_instagram") => {}
                Some("connections") => {
                    connections = Some(Connections::from_folder(&path)?);
                }
                Some("logged_information") => {}
                Some("media") => {
                    media = Some(Media::from_folder(&path)?);
                }
                Some("personal_information") => {}
                Some("preferences") => {}
                Some("security_and_login_information") => {}
                Some("your_instagram_activity") => {
                    activity = Some(Activity::try_from(path.as_ref())?);
                }
                _ => {
                    return Err(ParseError::UnexpectedFormat(format!(
                        "Found unexpected directory in Instagram archive: {:?}",
                        absolute(path)
                    )));
                }
            }
        }

        Ok(Self {
            connections,
            media,
            activity,
        })
    }

    // TODO: Move this method to a common Archive trait
    /// Save to file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), WriteError> {
        let path = path.as_ref();
        let file = std::fs::File::create(path)?;
        serde_json::to_writer_pretty(file, self)?;
        Ok(())
    }
}

impl TryFrom<&Path> for InstagramArchive {
    type Error = ParseError;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        if path.is_dir() {
            // Parse as root directory
            InstagramArchive::from_folder(path)
        } else if path
            .extension()
            .map(|ext| ext.to_str() == Some("zip"))
            .unwrap_or(false)
        {
            // Parse as zip file
            todo!()
        } else {
            Err(ParseError::UnexpectedFormat(
                format!("Expected a directory or zip file, found: {:?}", absolute(path)),
            ))
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
#[serde(deny_unknown_fields)]
pub struct LinkTimeValueData {
    pub href: String,
    pub value: Option<String>,
    pub timestamp: i32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
#[serde(deny_unknown_fields)]
pub struct LinkTimeData {
    pub href: String,
    pub timestamp: i32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
#[serde(deny_unknown_fields)]
pub struct LinkData {
    pub href: String,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
#[serde(deny_unknown_fields)]
pub struct Value {
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
#[serde(deny_unknown_fields)]
pub struct Timestamp {
    pub timestamp: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
#[serde(deny_unknown_fields)]
pub struct MediaUri {
    pub uri: String,
    pub creation_timestamp: Option<u64>,
    pub backup_uri: Option<String>,
}
