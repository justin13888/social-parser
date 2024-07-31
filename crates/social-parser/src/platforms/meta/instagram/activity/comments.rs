use std::{
    fs::File,
    io::BufReader,
    path::{absolute, Path},
};

use serde::{Deserialize, Serialize};

use crate::{
    common::ParseError,
    platforms::meta::instagram::{MediaUri, Timestamp, Value},
};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Comments {
    pub post_comments: Option<PostComments>,
    pub reel_comments: Option<ReelComments>,
}

impl TryFrom<&Path> for Comments {
    type Error = ParseError;

    /// Load from a directory. Assumes path is a directory.
    fn try_from(path: &Path) -> Result<Comments, Self::Error> {
        if !path.is_dir() {
            return Err(ParseError::UnexpectedFormat(format!(
                "Found unexpected non-directory in Comments: {:?}",
                path
            )));
        }

        let mut post_comments = None;
        let mut reel_comments = None;

        for entry in path.read_dir()? {
            let entry = entry?;
            let path = entry.path();

            // Must be a file
            if !path.is_file() {
                return Err(ParseError::UnexpectedFormat(format!(
                    "Found unexpected non-file in Comments: {:?}",
                    absolute(path)
                )));
            }

            match path.file_name().and_then(|s| s.to_str()) {
                Some("post_comments_1.json") => {
                    post_comments = Some(PostComments::try_from(path.as_path())?);
                }
                Some("reels_comments.json") => {
                    reel_comments = Some(ReelComments::try_from(path.as_path())?);
                }
                _ => {
                    return Err(ParseError::UnexpectedFormat(format!(
                        "Found unexpected file in Comments: {:?}",
                        absolute(path)
                    )));
                }
            }
        }

        Ok(Comments {
            post_comments,
            reel_comments,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
#[serde(deny_unknown_fields)]
pub struct PostComments(pub Vec<PostComment>);

impl TryFrom<&Path> for PostComments {
    type Error = ParseError;

    /// Load from a file. Assumes path is a file.
    fn try_from(path: &Path) -> Result<PostComments, Self::Error> {
        if !path.is_file() {
            return Err(ParseError::UnexpectedFormat(format!(
                "Found unexpected non-file in PostComments: {:?}",
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
pub struct PostComment {
    pub media_list_data: Vec<MediaUri>,
    pub string_map_data: CommentData,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
#[serde(deny_unknown_fields)]
pub struct CommentData {
    #[serde(rename = "Comment")]
    pub comment: Value,
    #[serde(rename = "Media Owner")]
    pub media_owner: Option<Value>,
    #[serde(rename = "Time")]
    pub time: Timestamp,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
#[serde(deny_unknown_fields)]
pub struct ReelComments {
    pub comments_reels_comments: Vec<ReelComment>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
#[serde(deny_unknown_fields)]
pub struct ReelComment {
    pub string_map_data: CommentData,
}

impl TryFrom<&Path> for ReelComments {
    type Error = ParseError;

    /// Load from a file. Assumes path is a file.
    fn try_from(path: &Path) -> Result<ReelComments, Self::Error> {
        if !path.is_file() {
            return Err(ParseError::UnexpectedFormat(format!(
                "Found unexpected non-file in ReelComments: {:?}",
                path
            )));
        }

        let file = File::open(path)?;
        let reader = BufReader::new(file);

        serde_json::from_reader(reader).map_err(|e| ParseError::Serde(path.to_owned(), e))
    }
}
