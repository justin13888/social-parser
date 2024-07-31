use std::{
    fs::File,
    io::BufReader,
    path::{absolute, Path},
};

use serde::{Deserialize, Serialize};

use crate::{
    common::ParseError,
    platforms::meta::instagram::{LinkData, LinkTimeData, LinkTimeValueData, Timestamp, Value},
};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Saved {
    pub saved_collections: Option<SavedCollections>,
    pub saved_posts: Option<SavedPosts>,
}

impl TryFrom<&Path> for Saved {
    type Error = ParseError;

    /// Load from a directory. Assumes path is a directory.
    fn try_from(path: &Path) -> Result<Saved, Self::Error> {
        if !path.is_dir() {
            return Err(ParseError::UnexpectedFormat(format!(
                "Found unexpected non-directory in Saved: {:?}",
                path
            )));
        }

        let mut saved_collections = None;
        let mut saved_posts = None;

        for entry in path.read_dir()? {
            let entry = entry?;
            let path = entry.path();

            // Must be a file
            if !path.is_file() {
                return Err(ParseError::UnexpectedFormat(format!(
                    "Found unexpected non-file in Saved: {:?}",
                    absolute(path)
                )));
            }

            match path.file_name().and_then(|s| s.to_str()) {
                Some("saved_collections.json") => {
                    saved_collections = Some(SavedCollections::try_from(path.as_path())?);
                }
                Some("saved_posts.json") => {
                    saved_posts = Some(SavedPosts::try_from(path.as_path())?);
                }
                _ => {
                    return Err(ParseError::UnexpectedFormat(format!(
                        "Found unexpected file in Saved: {:?}",
                        absolute(path)
                    )));
                }
            }
        }

        Ok(Saved {
            saved_collections,
            saved_posts,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
#[serde(deny_unknown_fields)]
pub struct SavedCollections {
    pub saved_saved_collections: Vec<SavedCollectionData>,
}

impl TryFrom<&Path> for SavedCollections {
    type Error = ParseError;

    /// Load from a file. Assumes path is a file.
    fn try_from(path: &Path) -> Result<SavedCollections, Self::Error> {
        if !path.is_file() {
            return Err(ParseError::UnexpectedFormat(format!(
                "Found unexpected non-file in SavedCollections: {:?}",
                path
            )));
        }

        let file = File::open(path)?;
        let reader = BufReader::new(file);

        serde_json::from_reader(reader).map_err(|e| ParseError::Serde(path.to_owned(), e))
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
#[serde(untagged)]
pub enum SavedCollectionData {
    Album(SavedCollectionAlbum),
    Entry(SavedCollectionEntry),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
#[serde(deny_unknown_fields)]
pub struct SavedCollectionAlbum {
    pub title: String,
    pub string_map_data: SavedCollectionAlbumData,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
#[serde(deny_unknown_fields)]
pub struct SavedCollectionAlbumData {
    #[serde(rename = "Name")]
    pub name: Value,
    #[serde(rename = "Creation Time")]
    pub creation_time: Timestamp,
    #[serde(rename = "Update Time")]
    pub update_time: Timestamp,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
#[serde(deny_unknown_fields)]
pub struct SavedCollectionEntry {
    pub string_map_data: SavedCollectionEntryData,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
#[serde(deny_unknown_fields)]
pub struct SavedCollectionEntryData {
    #[serde(rename = "Name")]
    pub name: LinkData,
    #[serde(rename = "Added Time")]
    pub added_time: Timestamp,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
#[serde(deny_unknown_fields)]
pub struct SavedPosts {
    pub saved_saved_media: Vec<SavedContent>,
}

impl TryFrom<&Path> for SavedPosts {
    type Error = ParseError;

    /// Load from a file. Assumes path is a file.
    fn try_from(path: &Path) -> Result<SavedPosts, Self::Error> {
        if !path.is_file() {
            return Err(ParseError::UnexpectedFormat(format!(
                "Found unexpected non-file in SavedPosts: {:?}",
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
pub struct SavedContent {
    pub title: String,
    pub string_map_data: SavedContentData,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
#[serde(deny_unknown_fields)]
pub struct SavedContentData {
    #[serde(rename = "Saved on")]
    saved_on: LinkTimeData,
}
