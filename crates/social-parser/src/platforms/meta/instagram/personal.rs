use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::common::ParseError;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Personal;

impl Personal {
    /// Load from a folder
    /// Assumes path is a directory.
    pub(super) fn from_folder<P: AsRef<Path>>(path: P) -> Result<Self, ParseError> {
        assert!(path.as_ref().is_dir());

        // TODO: Implement
        Ok(Personal)
    }
}
