use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::BufReader,
    path::{absolute, Path},
};

use crate::{common::ParseError, platforms::meta::instagram::StringData};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct FollowersNFollowing {
    pub favourite_accounts: Option<FavouriteAccounts>,
    pub blocked_accounts: Option<BlockedAccounts>,
    pub close_friends: Option<CloseFriends>,
    pub followers: Option<Followers>,
    pub following: Option<Following>,
    pub hide_story_from: Option<HideStoryFrom>,
    pub pending_follow_requests: Option<PendingFollowRequests>,
    pub recent_follow_requests: Option<RecentFollowRequests>,
    pub recently_unfollowed: Option<RecentlyUnfollowed>,
    pub removed_suggestions: Option<RemovedSuggestions>,
    pub restricted_accounts: Option<RestrictedAccounts>,
}

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

        let mut favourite_accounts = None;
        let mut blocked_accounts = None;
        let mut close_friends = None;
        let mut followers = None;
        let mut following = None;
        let mut hide_story_from = None;
        let mut pending_follow_requests = None;
        let mut recent_follow_requests = None;
        let mut recently_unfollowed = None;
        let mut removed_suggestions = None;
        let mut restricted_accounts = None;

        for entry in path.read_dir()? {
            let entry = entry?;
            let path = entry.path();

            // Must be a file
            if !path.is_file() {
                return Err(ParseError::UnexpectedFormat(format!(
                    "Found unexpected non-file in FollowersNFollowing: {:?}",
                    absolute(path)
                )));
            }

            match path.file_name().and_then(|s| s.to_str()) {
                Some("accounts_you've_favorited.json") => {
                    favourite_accounts = Some(FavouriteAccounts::try_from(path.as_path())?);
                }
                Some("blocked_accounts.json") => {
                    blocked_accounts = Some(BlockedAccounts::try_from(path.as_path())?);
                }
                Some("close_friends.json") => {
                    close_friends = Some(CloseFriends::try_from(path.as_path())?);
                }
                Some("followers_1.json") => {
                    followers = Some(Followers::try_from(path.as_path())?);
                }
                Some("following.json") => {
                    following = Some(Following::try_from(path.as_path())?);
                }
                Some("hide_story_from.json") => {
                    hide_story_from = Some(HideStoryFrom::try_from(path.as_path())?);
                }
                Some("pending_follow_requests.json") => {
                    pending_follow_requests =
                        Some(PendingFollowRequests::try_from(path.as_path())?);
                }
                Some("recent_follow_requests.json") => {
                    recent_follow_requests = Some(RecentFollowRequests::try_from(path.as_path())?);
                }
                Some("recently_unfollowed_accounts.json") => {
                    recently_unfollowed = Some(RecentlyUnfollowed::try_from(path.as_path())?);
                }
                Some("removed_suggestions.json") => {
                    removed_suggestions = Some(RemovedSuggestions::try_from(path.as_path())?);
                }
                Some("restricted_accounts.json") => {
                    restricted_accounts = Some(RestrictedAccounts::try_from(path.as_path())?);
                }
                _ => {
                    return Err(ParseError::UnexpectedFormat(format!(
                        "Found unexpected file in FollowersNFollowing: {:?}",
                        absolute(path)
                    )));
                }
            }
        }

        Ok(FollowersNFollowing {
            favourite_accounts,
            blocked_accounts,
            close_friends,
            followers,
            following,
            hide_story_from,
            pending_follow_requests,
            recent_follow_requests,
            recently_unfollowed,
            removed_suggestions,
            restricted_accounts,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(deny_unknown_fields)]
pub struct FavouriteAccounts {
    pub relationships_feed_favorites: Vec<FavouriteAccount>,
}

impl TryFrom<&Path> for FavouriteAccounts {
    type Error = ParseError;

    /// Load from a file. Assumes path is a file.
    fn try_from(path: &Path) -> Result<FavouriteAccounts, Self::Error> {
        if !path.is_file() {
            return Err(ParseError::UnexpectedFormat(format!(
                "Found unexpected non-file in FavouriteAccounts: {:?}",
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
pub struct FavouriteAccount {
    pub title: String,
    pub media_list_data: Vec<()>,
    pub string_list_data: Vec<StringData>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(deny_unknown_fields)]
pub struct BlockedAccounts {
    pub relationships_blocked_users: Vec<BlockedAccount>,
}

impl TryFrom<&Path> for BlockedAccounts {
    type Error = ParseError;

    /// Load from a file. Assumes path is a file.
    fn try_from(path: &Path) -> Result<BlockedAccounts, Self::Error> {
        if !path.is_file() {
            return Err(ParseError::UnexpectedFormat(format!(
                "Found unexpected non-file in BlockedAccounts: {:?}",
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
pub struct BlockedAccount {
    pub title: String,
    pub string_list_data: Vec<StringData>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(deny_unknown_fields)]
pub struct CloseFriends {
    pub relationships_close_friends: Vec<Relationship>,
}

impl TryFrom<&Path> for CloseFriends {
    type Error = ParseError;

    /// Load from a file. Assumes path is a file.
    fn try_from(path: &Path) -> Result<CloseFriends, Self::Error> {
        if !path.is_file() {
            return Err(ParseError::UnexpectedFormat(format!(
                "Found unexpected non-file in CloseFriends: {:?}",
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
pub struct Followers(pub Vec<Relationship>);

impl TryFrom<&Path> for Followers {
    type Error = ParseError;

    /// Load from a file. Assumes path is a file.
    fn try_from(path: &Path) -> Result<Followers, Self::Error> {
        if !path.is_file() {
            return Err(ParseError::UnexpectedFormat(format!(
                "Found unexpected non-file in Followers: {:?}",
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
pub struct Following {
    pub relationships_following: Vec<Relationship>,
}

impl TryFrom<&Path> for Following {
    type Error = ParseError;

    /// Load from a file. Assumes path is a file.
    fn try_from(path: &Path) -> Result<Following, Self::Error> {
        if !path.is_file() {
            return Err(ParseError::UnexpectedFormat(format!(
                "Found unexpected non-file in Following: {:?}",
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
pub struct HideStoryFrom {
    pub relationships_hide_stories_from: Vec<Relationship>,
}

impl TryFrom<&Path> for HideStoryFrom {
    type Error = ParseError;

    /// Load from a file. Assumes path is a file.
    fn try_from(path: &Path) -> Result<HideStoryFrom, Self::Error> {
        if !path.is_file() {
            return Err(ParseError::UnexpectedFormat(format!(
                "Found unexpected non-file in HideStoryFrom: {:?}",
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
pub struct PendingFollowRequests {
    pub relationships_follow_requests_sent: Vec<Relationship>,
}

impl TryFrom<&Path> for PendingFollowRequests {
    type Error = ParseError;

    /// Load from a file. Assumes path is a file.
    fn try_from(path: &Path) -> Result<PendingFollowRequests, Self::Error> {
        if !path.is_file() {
            return Err(ParseError::UnexpectedFormat(format!(
                "Found unexpected non-file in PendingFollowRequests: {:?}",
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
pub struct RecentFollowRequests {
    pub relationships_permanent_follow_requests: Vec<Relationship>,
}

impl TryFrom<&Path> for RecentFollowRequests {
    type Error = ParseError;

    /// Load from a file. Assumes path is a file.
    fn try_from(path: &Path) -> Result<RecentFollowRequests, Self::Error> {
        if !path.is_file() {
            return Err(ParseError::UnexpectedFormat(format!(
                "Found unexpected non-file in RecentFollowRequests: {:?}",
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
pub struct RecentlyUnfollowed {
    pub relationships_unfollowed_users: Vec<Relationship>,
}

impl TryFrom<&Path> for RecentlyUnfollowed {
    type Error = ParseError;

    /// Load from a file. Assumes path is a file.
    fn try_from(path: &Path) -> Result<RecentlyUnfollowed, Self::Error> {
        if !path.is_file() {
            return Err(ParseError::UnexpectedFormat(format!(
                "Found unexpected non-file in RecentlyUnfollowed: {:?}",
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
pub struct RemovedSuggestions {
    pub relationships_dismissed_suggested_users: Vec<Relationship>,
}

impl TryFrom<&Path> for RemovedSuggestions {
    type Error = ParseError;

    /// Load from a file. Assumes path is a file.
    fn try_from(path: &Path) -> Result<RemovedSuggestions, Self::Error> {
        if !path.is_file() {
            return Err(ParseError::UnexpectedFormat(format!(
                "Found unexpected non-file in RemovedSuggestions: {:?}",
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
pub struct RestrictedAccounts {
    pub relationships_restricted_users: Vec<Relationship>,
}

impl TryFrom<&Path> for RestrictedAccounts {
    type Error = ParseError;

    /// Load from a file. Assumes path is a file.
    fn try_from(path: &Path) -> Result<RestrictedAccounts, Self::Error> {
        if !path.is_file() {
            return Err(ParseError::UnexpectedFormat(format!(
                "Found unexpected non-file in RestrictedAccounts: {:?}",
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
pub struct Relationship {
    pub title: String,
    pub media_list_data: Vec<()>,
    pub string_list_data: Vec<StringData>,
}
