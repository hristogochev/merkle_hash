use std::path::PathBuf;

#[derive(Debug)]
pub enum DirIterError {
    /// Failed to get the metadata of a file while going over a directory
    Metadata {
        /// Path of the file whose metadata we were unable to get
        path: PathBuf,
        source: std::io::Error,
    },
    /// Failed to read a directory entry while going over a directory
    DirEntry {
        /// Path of the parent directory
        path: PathBuf,
        source: std::io::Error,
    },
    /// Failed to get the file type of a file while going over a directory
    FileType {
        /// Path of the file whose file type we were unable to get
        path: PathBuf,
        source: std::io::Error,
    },
    /// Failed to access a directory that we wanted to read
    ReadDir {
        /// Path of the directory we attempted to read
        path: PathBuf,
        source: std::io::Error,
    },
}

impl std::fmt::Display for DirIterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DirIterError::Metadata { path, source } => {
                write!(f, "Metadata error for '{:?}': {source}", path)
            }
            DirIterError::DirEntry { path, source } => {
                write!(f, "Dir entry error for '{:?}': {source}", path)
            }
            DirIterError::FileType { path, source } => {
                write!(f, "File type error for '{:?}': {source}", path)
            }
            DirIterError::ReadDir { path, source } => {
                write!(f, "Read dir error for '{:?}': {source}", path)
            }
        }
    }
}

impl std::error::Error for DirIterError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            DirIterError::Metadata { path: _, source } => Some(source),
            DirIterError::DirEntry { path: _, source } => Some(source),
            DirIterError::FileType { path: _, source } => Some(source),
            DirIterError::ReadDir { path: _, source } => Some(source),
        }
    }
}
