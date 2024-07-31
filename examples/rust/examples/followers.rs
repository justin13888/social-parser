use color_eyre::eyre::{OptionExt, Result};
use social_parser::platforms::meta::instagram::{
    connections::followersnfollowing::Relationship, InstagramArchive,
};
use std::{collections::HashSet, path};

// See who doesn't follow you back
fn main() -> Result<()> {
    color_eyre::install()?;

    let path = path::absolute("./data/instagram")?;
    println!("Parsing at: {:?}", path);

    // Load Instagram Archive
    let archive = InstagramArchive::try_from(path.as_path())?;
    // println!("{:#?}", archive);

    let fnf = archive
        .connections
        .ok_or_eyre("No connections found")?
        .followers_n_following
        .ok_or_eyre("No FNF")?;

    let followers = fnf
        .followers
        .ok_or_eyre("No followers")?
        .0
        .into_iter()
        .collect::<HashSet<Relationship>>();
    let following = fnf
        .following
        .ok_or_eyre("No following")?
        .relationships_following;

    let not_following_back: Vec<_> = following
        .into_iter()
        .filter(|r| !followers.contains(r))
        .flat_map(|r| r.string_list_data)
        .map(|s| (s.href, s.value))
        .collect();

    not_following_back.into_iter().for_each(|(href, value)| {
        println!("{}: {}", value.unwrap_or("???".to_string()), href);
    });

    Ok(())
}
