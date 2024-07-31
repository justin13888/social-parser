pub mod contacts;
pub mod followersnfollowing;

use contacts::Contacts;
use followersnfollowing::FollowersNFollowing;
use serde::{Deserialize, Serialize};
use std::path::{absolute, Path};

use crate::common::ParseError;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Connections {
    contacts: Option<Contacts>,
    followers_n_following: Option<FollowersNFollowing>,
}

impl Connections {
    /// Load from a folder
    /// Assumes path is a directory.
    pub(super) fn from_folder<P: AsRef<Path>>(path: P) -> Result<Self, ParseError> {
        assert!(path.as_ref().is_dir());

        // Load all directories in the directory
        let mut contacts = None;
        let mut followers_n_following = None;

        for entry in path.as_ref().read_dir()? {
            let entry = entry?;
            let path = entry.path();

            // Must be a directory
            if !path.is_dir() {
                return Err(ParseError::UnexpectedFormat(format!(
                    "Found unexpected non-directory in Connections: {:?}",
                    absolute(path)
                )));
            }

            match path.file_stem().and_then(|s| s.to_str()) {
                Some("contacts") => {
                    contacts = Some(Contacts::try_from(path.as_path())?);
                }
                Some("followers_and_following") => {
                    followers_n_following = Some(FollowersNFollowing::try_from(path.as_path())?);
                }
                _ => {
                    return Err(ParseError::UnexpectedFormat(format!(
                        "Found unexpected directory in Connections: {:?}",
                        absolute(path)
                    )));
                }
            }
        }

        Ok(Self {
            contacts,
            followers_n_following,
        })
    }
}
