mod files;

use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use chrono::{DateTime, TimeZone, Utc};
use exif::{Field, Tag, Value};
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

fn get_photo_date_time(path: &PathBuf) -> Option<DateTime<Utc>> {
    get_exif_date_time_original(path).or_else(|| get_file_last_modified_datetime(path))
}

fn get_exif_date_time_original(path: &PathBuf) -> Option<DateTime<Utc>> {
    let exifreader = exif::Reader::new();

    let file = File::open(path).unwrap();
    let mut bufreader = BufReader::new(file);
    let exif = exifreader.read_from_container(&mut bufreader).unwrap();

    exif.get_field(Tag::DateTimeOriginal, exif::In::PRIMARY)
        .map(ascii_to_date_time)
        .flatten()
}

fn ascii_to_date_time(field: &Field) -> Option<DateTime<Utc>> {
    match field.value {
        Value::Ascii(ref vec) => {
            let exif_date_time = exif::DateTime::from_ascii(vec[0].as_ref()).unwrap();

            Some(exif_to_date_time(exif_date_time))
        }
        _ => None,
    }
}

fn exif_to_date_time(exif_date_time: exif::DateTime) -> DateTime<Utc> {
    Utc.datetime_from_str(exif_date_time.to_string().as_str(), "%Y-%m-%d %H:%M:%S")
        .unwrap()
}

fn get_file_last_modified_datetime(path: &PathBuf) -> Option<DateTime<Utc>> {
    path.metadata()
        .ok()
        .map(|metadata| metadata.modified().ok())
        .flatten()
        .map(DateTime::from)
}

fn main() {
    let args: Cli = Cli::from_args();

    let files = files::get_images(&args.dir).unwrap();

    println!("Files:");
    for entry in &files {
        println!("{:?} - {:?}", get_photo_date_time(entry), entry)
    }
}
