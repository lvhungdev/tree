use std::fs;
use std::io::{Error, ErrorKind, Result};

pub struct PathUtils;

impl PathUtils {
    pub fn is_path_directory(path: &str) -> Result<bool> {
        let metadata = fs::metadata(path)?;
        return Ok(metadata.is_dir());
    }

    pub fn get_absolute_path(path: &str) -> Result<String> {
        let absolute_path: Result<String> = match fs::canonicalize(path)?.to_str() {
            Some(path) => Ok(String::from(path)),
            None => Err(Error::new(ErrorKind::Other, "Path is invalid")),
        };

        let mut absolute_path: String = absolute_path?;

        absolute_path = absolute_path.replace("\\\\?\\", "");
        absolute_path = absolute_path.replace("\\", "/");

        return Ok(absolute_path);
    }

    pub fn get_name_from_absolute_path(path: &str) -> Result<String> {
        let path: &str = if path.ends_with("/") {
            &path[..path.len() - 1]
        } else {
            path
        };

        return match path.rfind("/") {
            Some(index) => Ok(String::from(&path[index + 1..])),
            None => Err(Error::new(ErrorKind::Other, "Path is invalid")),
        };
    }
}
