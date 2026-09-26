use std::fmt::{Display, Formatter};
use std::path::{PathBuf, StripPrefixError};

/// Types of errors that can occur when recursively indexing a directory for its hashes.
#[derive(Debug)]
pub enum IndexingError {
    UnableToReadFileName {
        absolute_path: PathBuf,
    },
    UnableToReadFile {
        absolute_path: PathBuf,
        source: std::io::Error,
    },
    UnableToReadDir {
        absolute_path: PathBuf,
        source: std::io::Error,
    },
    UnableToReadDirEntry {
        parent_absolute_path: PathBuf,
        source: std::io::Error,
    },
    UnableToStripRootPrefix {
        absolute_path: PathBuf,
        root: String,
        source: StripPrefixError,
    },
}

impl Display for IndexingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            IndexingError::UnableToReadFileName { absolute_path } => {
                write!(f, "Unable to read file name: {:?}", absolute_path)
            }
            IndexingError::UnableToReadFile { absolute_path, source } => {
                write!(f, "Unable to read file: {:?}, error: {}", absolute_path, source)
            }
            IndexingError::UnableToReadDir {
                absolute_path,
                source,
            } => {
                write!(f, "Unable to read dir: {:?}, error: {}", absolute_path, source)
            }
            IndexingError::UnableToReadDirEntry {
                parent_absolute_path,
                source,
            } => {
                write!(
                    f,
                    "Unable to read dir entry in dir: {:?}, error: {}",
                    parent_absolute_path, source
                )
            }
            IndexingError::UnableToStripRootPrefix {
                absolute_path,
                root,
                source,
            } => {
                write!(
                    f,
                    "Unable to strip root prefix for path: {:?}, where root: {}, error: {}",
                    absolute_path, root, source
                )
            }
        }
    }
}

impl std::error::Error for IndexingError {}
