use std::fs::FileType;
use std::path::Path;
use std::{
    collections::VecDeque,
    fs::{self, ReadDir},
    path::PathBuf,
};

use crate::error::DirIterError;

pub struct DirIter {
    read_dirs: VecDeque<(PathBuf, ReadDir)>,
    root: Option<PathBuf>,
}

impl Iterator for DirIter {
    type Item = Result<(PathBuf, FileType), DirIterError>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(path) = self.root.take() {
            let metadata = match fs::metadata(&path) {
                Ok(metadata) => metadata,
                Err(source) => return Some(Err(DirIterError::Metadata { path, source })),
            };

            let file_type = metadata.file_type();
            if file_type.is_dir() {
                let new_read_dir = match fs::read_dir(&path) {
                    Ok(read_dir) => read_dir,
                    Err(source) => {
                        return Some(Err(DirIterError::ReadDir { path, source }));
                    }
                };
                self.read_dirs.push_front((path.clone(), new_read_dir));
            }

            return Some(Ok((path, file_type)));
        }

        let ((path, read_dir), item) = loop {
            let Some((path, mut read_dir)) = self.read_dirs.pop_front() else {
                break None;
            };

            let Some(item) = read_dir.next() else {
                continue;
            };

            break Some(((path, read_dir), item));
        }?;

        self.read_dirs.push_front((path.clone(), read_dir));

        let item = match item {
            Ok(item) => item,
            Err(source) => return Some(Err(DirIterError::DirEntry { path, source })),
        };

        let path = item.path();

        let file_type = match item.file_type() {
            Ok(file_type) => file_type,
            Err(source) => return Some(Err(DirIterError::FileType { path, source })),
        };

        if file_type.is_dir() {
            let new_read_dir = match fs::read_dir(&path) {
                Ok(read_dir) => read_dir,
                Err(source) => {
                    return Some(Err(DirIterError::ReadDir { path, source }));
                }
            };
            self.read_dirs.push_front((path.clone(), new_read_dir));
        }

        Some(Ok((path, file_type)))
    }
}

impl DirIter {
    pub fn new(root: impl AsRef<Path>) -> Self {
        let read_dirs: VecDeque<(PathBuf, ReadDir)> = VecDeque::new();
        let root = Some(root.as_ref().to_path_buf());
        Self { read_dirs, root }
    }
}
