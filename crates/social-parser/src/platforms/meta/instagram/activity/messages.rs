use std::{
    collections::HashMap,
    fs::File,
    io::BufReader,
    path::{absolute, Path},
};

use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::{common::ParseError, platforms::meta::instagram::MediaUri};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Messages {
    // photos: Photos, // TODO: Implement
    pub inbox: Option<Inbox>,
} // TODO

impl TryFrom<&Path> for Messages {
    type Error = ParseError;

    /// Load from a directory. Assumes path is a directory.
    fn try_from(path: &Path) -> Result<Messages, Self::Error> {
        if !path.is_dir() {
            return Err(ParseError::UnexpectedFormat(format!(
                "Found unexpected non-directory in Messages: {:?}",
                path
            )));
        }

        let mut inbox = None;

        for entry in path.read_dir()? {
            let entry = entry?;
            let path = entry.path();

            // if !path.is_file() {
            //     return Err(ParseError::UnexpectedFormat(format!(
            //         "Found unexpected non-file in Messages: {:?}",
            //         absolute(path)
            //     )));
            // }

            match path.file_name().and_then(|s| s.to_str()) {
                Some("inbox") => {
                    inbox = Some(Inbox::try_from(path.as_path())?);
                }
                Some("reported_conversations.json") | Some("secret_conversations.json") => {
                    if !path.is_file() {
                        return Err(ParseError::UnexpectedFormat(format!(
                            "Found unexpected non-file in Messages: {:?}",
                            absolute(path)
                        )));
                    }
                }
                Some("cross-app-inbox") | Some("message_requests") => {
                    if !path.is_dir() {
                        return Err(ParseError::UnexpectedFormat(format!(
                            "Found unexpected non-directory in Messages: {:?}",
                            absolute(path)
                        )));
                    }
                }
                Some("photos") => {} // TODO: Implement
                _ => {
                    return Err(ParseError::UnexpectedFormat(format!(
                        "Found unexpected file in Messages: {:?}",
                        absolute(path)
                    )));
                }
            }
        }

        Ok(Messages { inbox })
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Inbox(pub HashMap<String, Vec<InboxData>>);

impl TryFrom<&Path> for Inbox {
    type Error = ParseError;

    /// Load from a directory. Assumes path is a directory.
    fn try_from(path: &Path) -> Result<Inbox, Self::Error> {
        if !path.is_dir() {
            return Err(ParseError::UnexpectedFormat(format!(
                "Found unexpected non-directory in Inbox: {:?}",
                path
            )));
        }

        // Define the regex pattern
        let pattern = r"^message_.*\.json$";
        let re = Regex::new(pattern).expect("Invalid regex pattern");

        let inner = path
            .read_dir()?
            .map(|entry| {
                let entry = entry?;
                let path = entry.path();

                if !path.is_dir() {
                    return Err(ParseError::UnexpectedFormat(format!(
                        "Found unexpected non-directory in inbox: {:?}",
                        absolute(path)
                    )));
                }

                let binding = path.clone();
                let inbox_file_name =
                    binding
                        .file_name()
                        .and_then(|s| s.to_str())
                        .ok_or_else(|| {
                            ParseError::UnexpectedFormat(format!(
                                "Invalid inbox name: {:?}",
                                absolute(path.clone())
                            ))
                        })?;

                let parse_result = {
                    // Search for files inside
                    // There should only exist message_*.json
                    let mut message_json_paths = vec![];
                    for entry in path.read_dir()? {
                        let entry = entry?;
                        let path = entry.path();

                        match path.file_name().and_then(|s| s.to_str()) {
                            Some(filename) if re.is_match(filename) => {
                                if !path.is_file() {
                                    return Err(ParseError::UnexpectedFormat(format!(
                                        "Found unexpected non-file in inbox: {:?}",
                                        absolute(path)
                                    )));
                                }
                                message_json_paths.push(path);
                            }
                            Some("photos") => {} // TODO: See how to store the photos inside
                            Some("videos") => {} // TODO: See how to store the videos inside
                            Some("audio") => {}  // TODO: See how to store the audio inside
                            _ => {
                                return Err(ParseError::UnexpectedFormat(format!(
                                    "Found unexpected file in inbox: {:?}",
                                    absolute(path)
                                )));
                            }
                        }
                    }

                    if !message_json_paths.is_empty() {
                        message_json_paths
                            .into_iter()
                            .map(|p| InboxData::try_from(p.as_ref()))
                            .collect::<Result<Vec<_>, _>>()
                    } else {
                        Err(ParseError::UnexpectedFormat(format!(
                            "Expected message_1.json in inbox '{:?}': {:?}",
                            path.clone()
                                .file_name()
                                .and_then(|s| s.to_str())
                                .unwrap_or("N/A"),
                            absolute(path)
                        )))
                    }
                };

                match parse_result {
                    Ok(data) => Ok((inbox_file_name.to_owned(), data)),
                    Err(e) => Err(e),
                }
            })
            .collect::<Result<HashMap<_, _>, ParseError>>()?;

        Ok(Inbox(inner))
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(deny_unknown_fields)]
pub struct InboxData {
    pub participants: Vec<Participant>,
    pub messages: Vec<Message>,
    pub title: String,
    pub is_still_participant: bool,
    pub thread_path: String,
    pub magic_words: Vec<()>,
    pub image: Option<MediaUri>,
    pub joinable_mode: Option<JoinableMode>,
}

impl TryFrom<&Path> for InboxData {
    type Error = ParseError;

    /// Load from a file. Assumes path is a file.
    fn try_from(path: &Path) -> Result<InboxData, Self::Error> {
        if !path.is_file() {
            return Err(ParseError::UnexpectedFormat(format!(
                "Found unexpected non-file in InboxData: {:?}",
                path
            )));
        }

        let file = File::open(path)?;
        let reader = BufReader::new(file);

        serde_json::from_reader(reader).map_err(|e| ParseError::Serde(path.to_owned(), e))
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(deny_unknown_fields)]
pub struct Participant {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(deny_unknown_fields)]
pub struct Message {
    pub sender_name: String,
    pub timestamp_ms: u64,
    pub content: Option<String>,
    pub photos: Option<Vec<MediaUri>>,
    pub videos: Option<Vec<MediaUri>>,
    pub audio_files: Option<Vec<MediaUri>>,
    pub share: Option<Share>,
    pub call_duration: Option<u32>,
    pub is_unsent: Option<bool>,
    pub is_geoblocked_for_viewer: bool,
    pub reactions: Option<Vec<Reaction>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(deny_unknown_fields)]
pub struct Share {
    pub link: Option<String>,
    pub share_text: Option<String>,
    pub original_content_owner: Option<String>,
    pub profile_share_username: Option<String>,
    pub profile_share_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(deny_unknown_fields)]
pub struct Reaction {
    /// Emoji reaction in unicode
    pub reaction: String,
    /// User who reacted
    pub actor: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(deny_unknown_fields)]
pub struct JoinableMode {
    pub mode: u32,
    pub link: String,
}
