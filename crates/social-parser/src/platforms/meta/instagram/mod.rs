pub mod activity;
pub mod ads;
pub mod connections;
pub mod logged;
pub mod media;
pub mod offsite;
pub mod personal;
pub mod preferences;
pub mod security;

use std::path::{self, absolute, Path, PathBuf};

use activity::Activity;
use ads::Ads;
use connections::Connections;
use logged::Logged;
use media::Media;
use offsite::Offsite;
use personal::Personal;
use preferences::Preferences;
use security::Security;
use serde::{Deserialize, Serialize};

use crate::common::{ParseError, WriteError};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InstagramArchive {
    pub ads: Option<Ads>,
    pub offsite: Option<Offsite>,
    pub connections: Option<Connections>,
    pub logged: Option<Logged>,
    pub media: Option<Media>,
    pub personal: Option<Personal>,
    pub preferences: Option<Preferences>,
    pub security: Option<Security>,
    pub activity: Option<Activity>,
}

impl InstagramArchive {
    /// Load from a folder
    /// Assumes path is a directory.
    fn from_folder<P: AsRef<Path>>(path: P) -> Result<Self, ParseError> {
        assert!(path.as_ref().is_dir());

        // Load all directories in the directory
        let mut ads = None;
        let mut connections = None;
        let mut logged = None;
        let mut media = None;
        let mut offsite = None;
        let mut personal = None;
        let mut preferences = None;
        let mut security = None;
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
                Some("ads_information") => {
                    ads = Some(Ads::from_folder(&path)?);
                }
                Some("apps_and_websites_off_of_instagram") => {
                    offsite = Some(Offsite::from_folder(&path)?);
                }
                Some("connections") => {
                    connections = Some(Connections::from_folder(&path)?);
                }
                Some("logged_information") => {
                    logged = Some(Logged::from_folder(&path)?);
                }
                Some("media") => {
                    media = Some(Media::from_folder(&path)?);
                }
                Some("personal_information") => {
                    personal = Some(Personal::from_folder(&path)?);
                }
                Some("preferences") => {
                    preferences = Some(Preferences::from_folder(&path)?);
                }
                Some("security_and_login_information") => {
                    security = Some(Security::from_folder(&path)?);
                }
                Some("your_instagram_activity") => {
                    activity = Some(Activity::from_folder(&path)?);
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
            ads,
            offsite,
            connections,
            logged,
            media,
            personal,
            preferences,
            security,
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
                "Unknown file type".to_string(),
            ))
        }
    }
}
