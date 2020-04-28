use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use chrono::{DateTime, TimeZone, Utc};
use exif::{Field, Tag, Value};

pub fn get_shooting_datetime(path: &PathBuf) -> Option<DateTime<Utc>> {
    get_exif_datetime_original(path).or_else(|| get_file_last_modified_datetime(path))
}

fn get_exif_datetime_original(path: &PathBuf) -> Option<DateTime<Utc>> {
    let exifreader = exif::Reader::new();

    let file = File::open(path).unwrap();
    let mut bufreader = BufReader::new(file);
    let exif = exifreader.read_from_container(&mut bufreader).unwrap();

    exif.get_field(Tag::DateTimeOriginal, exif::In::PRIMARY)
        .map(ascii_to_datetime)
        .flatten()
}

fn ascii_to_datetime(field: &Field) -> Option<DateTime<Utc>> {
    match field.value {
        Value::Ascii(ref vec) => {
            let exif_datetime = exif::DateTime::from_ascii(vec[0].as_ref()).unwrap();

            Some(exif_to_datetime(exif_datetime))
        }
        _ => None,
    }
}

fn exif_to_datetime(exif_datetime: exif::DateTime) -> DateTime<Utc> {
    Utc.datetime_from_str(exif_datetime.to_string().as_str(), "%Y-%m-%d %H:%M:%S")
        .unwrap()
}

fn get_file_last_modified_datetime(path: &PathBuf) -> Option<DateTime<Utc>> {
    path.metadata()
        .ok()
        .map(|metadata| metadata.modified().ok())
        .flatten()
        .map(DateTime::from)
}
