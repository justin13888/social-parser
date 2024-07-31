use color_eyre::eyre::{OptionExt, Result};
use social_parser::platforms::meta::instagram::{
    connections::followersnfollowing::Relationship, InstagramArchive,
};
use std::{
    collections::{HashMap, HashSet},
    path,
};

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
        .flat_map(|r| r.string_list_data)
        .map(|s| (s.href, s.value))
        .collect::<HashMap<String, Option<String>>>();
    let following = fnf
        .following
        .ok_or_eyre("No following")?
        .relationships_following;

    let not_following_back: Vec<_> = following
        .into_iter()
        .flat_map(|r| r.string_list_data)
        .map(|s| (s.href, s.value))
        .filter(|(following_href, _following_value)| !followers.contains_key(following_href))
        .collect();

    not_following_back.iter().for_each(|(href, value)| {
        println!("{}: {}", value.clone().unwrap_or("???".to_string()), href);
    });
    println!("\n{} don't follow you back", not_following_back.len());

    Ok(())
}
