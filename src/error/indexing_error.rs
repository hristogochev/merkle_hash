use std::fmt::{Display, Formatter};
use std::io;
use std::path::{PathBuf, StripPrefixError};

 #[cfg(feature = "camino")]
use camino::Utf8PathBuf;

/// Types of errors that can occur when recursively indexing a directory for its hashes.
#[derive(Debug)]
pub enum IndexingError {
    PathIsNotValidUtf8(PathBuf),
     #[cfg(feature = "camino")]
    UnableToReadFileName(Utf8PathBuf),
     #[cfg(feature = "camino")]
    UnableToReadFile(Utf8PathBuf, io::Error),
     #[cfg(feature = "camino")]
    UnableToReadDir(Utf8PathBuf, io::Error),
     #[cfg(feature = "camino")]
    UnableToReadDirEntry(Utf8PathBuf, io::Error),
     #[cfg(feature = "camino")]
     UnableToStripRootPrefix(Utf8PathBuf, String, StripPrefixError),
     #[cfg(not(feature = "camino"))]
    UnableToReadFileName(PathBuf),
     #[cfg(not(feature = "camino"))]
    UnableToReadFile(PathBuf, io::Error),
     #[cfg(not(feature = "camino"))]
    UnableToReadDir(PathBuf, io::Error),
     #[cfg(not(feature = "camino"))]
    UnableToReadDirEntry(PathBuf, io::Error),
     #[cfg(not(feature = "camino"))]
    UnableToStripRootPrefix(PathBuf, String, StripPrefixError),
}

impl Display for IndexingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            IndexingError::PathIsNotValidUtf8(path) => {
                write!(f, "Path is not valid UTF8 path: {:?}", path)
            }
            IndexingError::UnableToReadFileName(path) => {
                write!(f, "Unable to read file name: {:?}", path)
            }
            IndexingError::UnableToReadFile(path, error) => {
                write!(f, "Unable to read file: {:?}, error: {}", path, error)
            }
            IndexingError::UnableToReadDir(path, error) => {
                write!(f, "Unable to read dir: {:?}, error: {}", path, error)
            }
            IndexingError::UnableToReadDirEntry(path, error) => {
                write!(f, "Unable to read dir entry in dir: {:?}, error: {}", path, error)
            }
            IndexingError::UnableToStripRootPrefix(path, root, error) => {
                write!(f, "Unable to strip root prefix for path: {:?}, where root: {}, error: {}", path, root, error)
            }
        }
    }
}


impl std::error::Error for IndexingError {}



