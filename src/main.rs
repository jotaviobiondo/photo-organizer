use std::fs::{DirEntry};
use std::path::{PathBuf};

use structopt::StructOpt;


#[derive(Debug, StructOpt)]
#[structopt(name = "photo-organizer", about = "Organize photos of a folder by year/month")]
struct Cli {
    #[structopt(default_value = ".", parse(from_os_str))]
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


fn main() {
    let args: Cli = Cli::from_args();

    println!("{:?}", args);

    let Cli { dir } = args;

    let files = find_image_files_from_current_dir(&dir).unwrap();

    println!("Files:");
    for entry in &files {
        println!("{}", entry.display())
    }
}
