use color_eyre::eyre::Result;
use social_parser::platforms::meta::instagram::InstagramArchive;
use std::path;

fn main() -> Result<()> {
    color_eyre::install()?;

    let path = path::absolute("./data/instagram")?;
    println!("Parsing at: {:?}", path);

    // Load Instagram Archive
    let archive = InstagramArchive::try_from(path.as_path())?;
    // println!("{:#?}", archive);

    // Do something

    // e.g. Save to file
    archive.save_to_file("./data/instagram.json")?;

    Ok(())
}
