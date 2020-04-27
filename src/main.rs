use std::fs::{DirEntry, File};
use std::io::BufReader;
use std::path::{Path, PathBuf};

use chrono::{DateTime, TimeZone, Utc};
use exif::{Field, Tag, Value};
use structopt::StructOpt;

const IMAGE_EXTENSIONS: [&str; 3] = ["jpg", "jpeg", "png"];

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

fn find_image_files_from_current_dir(dir: &PathBuf) -> Result<Vec<PathBuf>, String> {
    dir.read_dir()
        .map(|files| {
            files
                .flatten()
                .map(|entry: DirEntry| entry.path())
                .filter(|path| path.is_file())
                .filter(is_image)
                .collect()
        })
        .map_err(|err| err.to_string())
}

fn is_image(path: &PathBuf) -> bool {
    path.extension()
        .map(|extension| extension.to_str())
        .flatten()
        .map(|extension| extension.to_lowercase())
        .filter(|extension| IMAGE_EXTENSIONS.contains(&extension.as_str()))
        .is_some()
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

    let files = find_image_files_from_current_dir(&args.dir).unwrap();

    println!("Files:");
    for entry in &files {
        println!("{:?} - {:?}", get_photo_date_time(entry), entry)
    }
}
