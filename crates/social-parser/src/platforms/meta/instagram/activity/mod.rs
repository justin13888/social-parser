pub mod comments;
pub mod likes;
pub mod messages;
pub mod saved;
pub mod threads;

use std::path::{absolute, Path};

use comments::Comments;
use likes::Likes;
use messages::Messages;
use saved::Saved;
use serde::{Deserialize, Serialize};
use threads::Threads;

use crate::common::ParseError;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Activity {
    pub comments: Option<Comments>,
    pub likes: Option<Likes>,
    pub messages: Option<Messages>,
    pub saved: Option<Saved>,
    pub threads: Option<Threads>,
}

impl TryFrom<&Path> for Activity {
    type Error = ParseError;

    /// Load from a directory. Assumes path is a directory.
    fn try_from(path: &Path) -> Result<Activity, Self::Error> {
        if !path.is_dir() {
            return Err(ParseError::UnexpectedFormat(format!(
                "Found unexpected non-directory in Activity: {:?}",
                path
            )));
        }

        let mut comments = None;
        let mut likes = None;
        let mut messages = None;
        let mut saved = None;
        let mut threads: Option<Threads> = None;

        for entry in path.read_dir()? {
            let entry = entry?;
            let path = entry.path();

            // Must be a directory
            if !path.is_dir() {
                return Err(ParseError::UnexpectedFormat(format!(
                    "Found unexpected non-directory in Activity: {:?}",
                    absolute(path)
                )));
            }

            match path.file_name().and_then(|s| s.to_str()) {
                Some("comments") => {
                    comments = Some(Comments::try_from(path.as_ref())?);
                }
                Some("content") => {}
                Some("events") => {}
                Some("fundraisers") => {}
                Some("gifts") => {}
                Some("instagram_live") => {}
                Some("likes") => {
                    likes = Some(Likes::try_from(path.as_ref())?);
                }
                Some("media") => {
                    // todo!("Parsing for media not implemented yet");
                }
                Some("messages") => {
                    messages = Some(Messages::try_from(path.as_ref())?);
                }
                Some("meta_spark") => {}
                Some("monetization") => {}
                Some("other_activity") => {}
                Some("reports") => {}
                Some("saved") => {
                    saved = Some(Saved::try_from(path.as_ref())?);
                }
                Some("shopping") => {}
                Some("story_sticker_interactions") => {}
                Some("subscriptions") => {}
                Some("threads") => {
                    // threads = Some(Threads::try_from(path.as_ref())?); // TODO: Implement
                }
                Some("avatars_store") => {
                    // todo!("Parsing for avatars store not implemented yet");
                }
                Some("story_interactions") => {
                    // todo!("Parsing for story interactions not implemented yet");
                }
                _ => {
                    return Err(ParseError::UnexpectedFormat(format!(
                        "Found unexpected file in Activity: {:?}",
                        absolute(path)
                    )));
                }
            }
        }

        Ok(Activity {
            comments,
            likes,
            messages,
            saved,
            threads,
        })
    }
}
