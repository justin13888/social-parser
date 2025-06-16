//! A simple example to parse Instagram messages of a specific ID from an archive.

use chrono::prelude::*;
use color_eyre::eyre::{OptionExt, Result};
use social_parser::platforms::meta::instagram::InstagramArchive;
use std::path;

// See a summary of past messages you talked to
fn main() -> Result<()> {
    color_eyre::install()?;

    let path = path::absolute("./data/instagram")?;
    println!("Parsing at: {path:?}");
    assert!(path.is_dir(), "Path must be a directory");

    // Load Instagram Archive
    let archive = InstagramArchive::try_from(path.as_path())?;
    // println!("{:#?}", archive);

    let messages = archive
        .activity
        .ok_or_eyre("No activity")?
        .messages
        .ok_or_eyre("No messages")?
        .inbox
        .ok_or_eyre("No inbox")?
        .0;

    let inboxes = messages
        .get("TODO: ADD YOUR MESSAGE ID HERE")
        .ok_or_eyre("Message ID not found")?;

    for inbox in inboxes.iter() {
        println!("{}\n", inbox.title);

        let mut messages = inbox
            .messages
            .iter()
            .flat_map(|message| {
                if message.is_unsent != Some(true) {
                    let text = format!(
                        "{} {}",
                        message.content.as_deref().unwrap_or(""),
                        message
                            .reactions
                            .as_ref()
                            .map(|r| r
                                .iter()
                                .map(|r| r.reaction.clone())
                                .collect::<Vec<_>>()
                                .join(", "))
                            .unwrap_or_default(),
                    );

                    // if let Some(content) = &message.content {
                    //     content
                    // } else if let Some(photos) = &message.photos {
                    //     photos
                    // } else {
                    //     ""
                    // };

                    Some((message.sender_name.clone(), message.timestamp_ms, text))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        messages.sort_by_key(|x| x.1);

        for message in messages.iter() {
            let (sender_name, timestamp_ms, text) = message;
            // Convert milliseconds to seconds and nanoseconds
            let seconds = timestamp_ms / 1000;
            let nanoseconds = (timestamp_ms % 1000) * 1_000_000;

            // Create a DateTime<Local> from timestamp
            let date_time: DateTime<Local> = Local
                .timestamp_opt(seconds.try_into().unwrap(), nanoseconds as u32)
                .unwrap();
            println!(
                "{} [{}]: {}",
                sender_name,
                date_time.format("%Y-%m-%d %H:%M:%S %Z"),
                text
            );
        }
        // TODO: Doesn't display image
    }

    Ok(())
}
