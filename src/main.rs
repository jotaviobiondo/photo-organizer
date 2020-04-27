use std::fs::{DirEntry, File};
use std::io::BufReader;
use std::path::{Path, PathBuf};

use chrono::{DateTime, TimeZone, Utc};
use exif::{Field, Tag, Value};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "photo-organizer", about = "Organize photos of a folder by year/month")]
struct Cli {
    #[structopt(
    help = "The directory containing the photos to organize. Default is the current directory.",
    default_value = ".",
    parse(from_os_str)
    )]
    dir: PathBuf
}


fn has_extensions(path: &PathBuf, extensions: &Vec<&str>) -> bool {
    path.extension()
        .map(|x| x.to_str())
        .flatten()
        .map(|x| x.to_lowercase())
        .filter(|x| extensions.contains(&x.as_str()))
        .is_some()
}

fn find_image_files_from_current_dir(dir: &PathBuf) -> Result<Vec<PathBuf>, String> {
    let extensions = vec!["jpg", "jpeg", "png"];

    dir.read_dir()
        .map(|files| {
            files
                .flatten()
                .map(|x: DirEntry| x.path())
                .filter(|x| x.is_file())
                .filter(|x| has_extensions(x, &extensions))
                .collect()
        })
        .map_err(|err| err.to_string())
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

fn exif_to_date_time(exif_date_time: exif::DateTime) -> DateTime<Utc> {
    Utc.datetime_from_str(exif_date_time.to_string().as_str(), "%Y-%m-%d %H:%M:%S").unwrap()
}

fn ascii_to_date_time(field: &Field) -> Option<DateTime<Utc>> {
    match field.value {
        Value::Ascii(ref vec) => {
            let exif_date_time = exif::DateTime::from_ascii(vec[0].as_ref()).unwrap();

            Some(exif_to_date_time(exif_date_time))
        }
        _ => None
    }
}

fn get_file_last_modified_datetime(path: &PathBuf) -> Option<DateTime<Utc>> {
    path.metadata().ok()
        .map(|metadata| metadata.modified().ok())
        .flatten()
        .map(|time_modified| DateTime::from(time_modified))
}

fn get_photo_date_time(path: &PathBuf) -> Option<DateTime<Utc>> {
    get_exif_date_time_original(path)
        .or(get_file_last_modified_datetime(path))
}

fn main() {
    let args: Cli = Cli::from_args();

    println!("{:?}", args);

    let Cli { dir } = args;

    let files = find_image_files_from_current_dir(&dir).unwrap();

    println!("Files:");
    for entry in &files {
        println!("{:?} - {:?}", get_photo_date_time(entry), entry)
    }
}
