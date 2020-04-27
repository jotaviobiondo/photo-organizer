use std::fs::DirEntry;
use std::path::PathBuf;

const IMAGE_EXTENSIONS: [&str; 4] = ["jpg", "jpeg", "png", "tiff"];

pub fn get_images(dir: &PathBuf) -> Result<Vec<PathBuf>, String> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_image() {
        assert_eq!(is_image(&PathBuf::from("test.png")), true);
        assert_eq!(is_image(&PathBuf::from("test.PNG")), true);
        assert_eq!(is_image(&PathBuf::from("test.jpeg")), true);
        assert_eq!(is_image(&PathBuf::from("test.JPEG")), true);
        assert_eq!(is_image(&PathBuf::from("test.jpg")), true);
        assert_eq!(is_image(&PathBuf::from("test.JPG")), true);
        assert_eq!(is_image(&PathBuf::from("test.TIFF")), true);
        assert_eq!(is_image(&PathBuf::from("test.TIFF")), true);
        assert_eq!(is_image(&PathBuf::from("")), false);
        assert_eq!(is_image(&PathBuf::from("test.txt")), false);
        assert_eq!(is_image(&PathBuf::from("test")), false);
        assert_eq!(is_image(&PathBuf::from("png")), false);
        assert_eq!(is_image(&PathBuf::from("jpeg")), false);
        assert_eq!(is_image(&PathBuf::from("jpg")), false);
        assert_eq!(is_image(&PathBuf::from("tiff")), false);
    }
}
