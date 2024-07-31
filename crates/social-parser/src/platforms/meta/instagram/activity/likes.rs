use std::{
    fs::File,
    io::BufReader,
    path::{absolute, Path},
};

use serde::{Deserialize, Serialize};

use crate::{common::ParseError, platforms::meta::instagram::LinkTimeValueData};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Likes {
    pub liked_comments: Option<LikedComments>,
    pub liked_posts: Option<LikedPosts>,
}

impl TryFrom<&Path> for Likes {
    type Error = ParseError;

    /// Load from a directory. Assumes path is a directory.
    fn try_from(path: &Path) -> Result<Likes, Self::Error> {
        if !path.is_dir() {
            return Err(ParseError::UnexpectedFormat(format!(
                "Found unexpected non-directory in Likes: {:?}",
                path
            )));
        }

        let mut liked_comments = None;
        let mut liked_posts = None;

        for entry in path.read_dir()? {
            let entry = entry?;
            let path = entry.path();

            // Must be a file
            if !path.is_file() {
                return Err(ParseError::UnexpectedFormat(format!(
                    "Found unexpected non-file in Likes: {:?}",
                    absolute(path)
                )));
            }

            match path.file_name().and_then(|s| s.to_str()) {
                Some("liked_comments.json") => {
                    liked_comments = Some(LikedComments::try_from(path.as_path())?);
                }
                Some("liked_posts.json") => {
                    liked_posts = Some(LikedPosts::try_from(path.as_path())?);
                }
                _ => {
                    return Err(ParseError::UnexpectedFormat(format!(
                        "Found unexpected file in Likes: {:?}",
                        absolute(path)
                    )));
                }
            }
        }

        Ok(Likes {
            liked_comments,
            liked_posts,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
#[serde(deny_unknown_fields)]
pub struct LikedComments {
    pub likes_comment_likes: Vec<LikedContent>,
}

impl TryFrom<&Path> for LikedComments {
    type Error = ParseError;

    /// Load from a file. Assumes path is a file.
    fn try_from(path: &Path) -> Result<LikedComments, Self::Error> {
        if !path.is_file() {
            return Err(ParseError::UnexpectedFormat(format!(
                "Found unexpected non-file in LikedComments: {:?}",
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
pub struct LikedPosts {
    pub likes_media_likes: Vec<LikedContent>,
}

impl TryFrom<&Path> for LikedPosts {
    type Error = ParseError;

    /// Load from a file. Assumes path is a file.
    fn try_from(path: &Path) -> Result<LikedPosts, Self::Error> {
        if !path.is_file() {
            return Err(ParseError::UnexpectedFormat(format!(
                "Found unexpected non-file in LikedPosts: {:?}",
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
pub struct LikedContent {
    pub title: Option<String>,
    pub string_list_data: Vec<LinkTimeValueData>,
}
