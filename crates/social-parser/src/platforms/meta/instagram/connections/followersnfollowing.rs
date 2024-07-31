use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::common::ParseError;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FollowersNFollowing; // TODO: Complete

impl TryFrom<&Path> for FollowersNFollowing {
    type Error = ParseError;

    /// Load from a file. Assumes path is a file.
    fn try_from(path: &Path) -> Result<FollowersNFollowing, Self::Error> {
        if !path.is_dir() {
            return Err(ParseError::UnexpectedFormat(format!(
                "Found unexpected non-directory in FollowersNFollowing: {:?}",
                path
            )));
        }

        // TODO: Implement
        Ok(FollowersNFollowing)
    }
}
