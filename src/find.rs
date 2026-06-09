use std::{
    env,
    fs::{self, File},
    io::{self, ErrorKind},
    path::{Path, PathBuf},
};

use crate::errors::{Error, Result};

pub struct Finder<'a> {
    filename: &'a Path,
}

impl<'a> Finder<'a> {
    pub fn new() -> Self {
        Finder {
            filename: Path::new(".env"),
        }
    }

    pub fn filename(mut self, filename: &'a Path) -> Self {
        self.filename = filename;
        self
    }

    pub fn find(self) -> Result<(PathBuf, String)> {
        // TODO: change String to Iter Type
        // get the current directory and the specified file name and find the file
        let path = find(&env::current_dir().map_err(Error::Io)?, self.filename)?;
        let file = File::open(&path).map_err(Error::Io)?;
        // TODO: read the file using iter
        todo!()
    }
}

// Searches for "filename" in `directory` and parent diretories until found or root is reached
pub fn find(directory: &Path, filename: &Path) -> Result<PathBuf> {
    let candidate = directory.join(filename);

    // check if candidate is a file (The file) return the directory else throw an error
    match fs::metadata(&candidate) {
        Ok(metadata) => {
            if metadata.is_file() {
                return Ok(candidate);
            }
        }
        Err(error) => {
            if error.kind() == ErrorKind::NotFound {
                return Err(Error::Io(error));
            }
        }
    }

    // if its a directory, recursive find another file
    if let Some(parent) = directory.parent() {
        find(parent, filename)
    } else {
        Err(Error::Io(io::Error::new(
            io::ErrorKind::NotFound,
            "path not found",
        )))
    }
}
