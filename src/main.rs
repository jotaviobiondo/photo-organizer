mod files;
mod organizer;
mod photo;

use std::path::{Path, PathBuf};

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "photo-organizer",
    about = "Organize photos of a folder by year/month"
)]
struct Cli {
    #[structopt(
        help = "The directory containing the photos to organize. Default is the current directory.",
        default_value = ".",
        parse(from_os_str),
        validator(dir_validator)
    )]
    dir: PathBuf,
}

fn dir_validator(dir_str: String) -> Result<(), String> {
    let dir = Path::new(dir_str.as_str());

    if !dir.exists() {
        Err(format!("The directory '{}' does not exist.", dir.display()))
    } else if !dir.is_dir() {
        Err(format!("'{}' is not a directory.", dir.display()))
    } else {
        Ok(())
    }
}

fn main() {
    let args: Cli = Cli::from_args();

    let files = files::get_images(&args.dir).unwrap();

    println!("Files:");
    for entry in &files {
        println!("{:?} - {:?}", photo::get_shooting_datetime(entry), entry)
    }
}
