use std::str;
use crate::nolan_errors::IndexError;
use log::error;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::ErrorKind;
use std::io::SeekFrom;

/// Basic data structure that holds our data in our index on disk
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]
pub struct Entry {
    pub start: u32,
    pub total: u32,
}

pub struct Index {
    pub file_name: String,
    entries: Vec<Entry>,
    index_file: File,
}

impl Index {
    /// Create a new index given a file path
    pub fn new(index_path: &str) -> Result<Index, IndexError> {
        let error_message = format!("Unable to create and open file {}", index_path);
        let file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .append(true)
            .open(index_path.clone())
            .map_err(|e| {
                error!("{}", e);
                IndexError::new(&error_message)
            })?;
        let empty_entry_vec = Vec::new();
        Ok(Index {
            file_name: index_path.to_string(),
            entries: empty_entry_vec,
            index_file: file,
        })
    }

    
    /// Load the index from disk into memory
    pub fn load_index(&mut self) -> Result<u16, IndexError> {
        self.index_file.seek(SeekFrom::Start(0)).map_err(|e| {
            error!("{}", e);
            IndexError::new("unable seek to begining of the index")
        })?;
        let mut circut_break: bool = false;
        loop {
            let mut buffer = [0; 8];
            match self.index_file.read_exact(&mut buffer) {
                Ok(_) => {},
                Err(error) => {
                    if error.kind() == ErrorKind::UnexpectedEof {
                    circut_break = true;
                } else {
                    return Err(IndexError::new("unable seek to begining of the index"));
                }},
            }
            if circut_break {
                break;
            }
            let decoded_entry: Entry = bincode::deserialize(&buffer).map_err(|e| {
                error!("{}", e);
                IndexError::new("unable to deserialize entry")
            })?;
            self.entries.push(decoded_entry);
        }
        let value = u16::try_from(self.entries.len()).map_err(|e| {
            error!("{}", e);
            IndexError::new("unable to convert usize to u16")
        })?;
        Ok(value)
    }

    /// Given an offset, return the entry start and total bytes
    pub fn return_entry_details_by_offset(
        &self,
        offset: usize,
    ) -> Result<(u64, usize), IndexError> {
        if offset >= self.entries.len() {
            return Err(IndexError::new("offset is greater than entries length"));
        }
        let entry = self.entries[offset];
        let start_offset: u64 = entry.start.into();
        let total_bytes: usize = usize::try_from(entry.total).map_err(|e| {
            error!("{}", e);
            IndexError::new("unable to convert from u32 to usize")
        })?;
        Ok((start_offset, total_bytes))
    }
}

#[cfg(test)]
mod index_tests {
    use std::str;
    use std::path::Path;
    use tempdir::TempDir;
    use rand::{distributions::Alphanumeric, Rng}; // 0.8
    use crate::index::Index;
    use crate::nolan_errors::IndexError;
    use crate::utils;
    use crate::virtual_segment::VirtualSegment;

    fn create_index_file(test_dir_path: &str, message_to_write: &[u8]) -> String{
        let mut vs = VirtualSegment::new(test_dir_path, 100, 0);
        vs
            .write(message_to_write)
            .expect("unable to write data to virtual segment");
        vs.flush().expect("Unable to flush");
        let file_name = vs.full_log_path.clone();
        let thing = str::strip_suffix(&file_name, utils::LOG_SUFFIX).expect("unable to strip");
        let index_file_name = format!("{}{}", thing, utils::INDEX_SUFFIX);
        return index_file_name
    }

    #[test]
    fn test_new_index() {
        let tmp_dir = TempDir::new("test").expect("Unable to create temp directory");
        let test_dir_path = tmp_dir
            .path()
            .to_str()
            .expect("Unable to convert path to string");
        let s: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();
        let index_file_path = format!("{}{}{}", test_dir_path.to_string(), &s, utils::INDEX_SUFFIX);
        let index = Index::new(&index_file_path).expect("Error creating index");  
        //Check if the index file exists
        assert!(Path::new(&index.file_name).exists());
    }

    #[test]
    fn test_load_index() {
        let tmp_dir = TempDir::new("test").expect("Unable to create temp directory");
        let test_dir_path = tmp_dir
            .path()
            .to_str()
            .expect("Unable to convert path to string");
        let index_file_name = create_index_file(test_dir_path, "hello".as_bytes());

        let mut index = Index::new(&index_file_name).expect("Error creating index");
        index.load_index().expect("unable to load index");
        assert!(index.entries.len() == 1);
    }

    #[test]
    fn return_entry_details_by_offset() {
        let tmp_dir = TempDir::new("test").expect("Unable to create temp directory");
        let test_dir_path = tmp_dir
            .path()
            .to_str()
            .expect("Unable to convert path to string");
        let message = "hello";
        let index_file_name = create_index_file(test_dir_path, message.as_bytes());

        let mut index = Index::new(&index_file_name).expect("Error creating index");
        index.load_index().expect("unable to load index");

        let (start, total) = index.return_entry_details_by_offset(0).expect("Unable to get entry details");
        assert!(start == 0);
        assert!(total == message.len());
    }

    #[test]
    fn return_entry_details_by_offset_dne() {
        let tmp_dir = TempDir::new("test").expect("Unable to create temp directory");
        let test_dir_path = tmp_dir
            .path()
            .to_str()
            .expect("Unable to convert path to string");
        let message = "hello";
        let index_file_name = create_index_file(test_dir_path, message.as_bytes());
        let mut index = Index::new(&index_file_name).expect("Error creating index");
        index.load_index().expect("unable to load index");

        let index_error = index.return_entry_details_by_offset(1).unwrap_err();
        let wanted_error =
            IndexError::new("offset is greater than entries length");
        assert_eq!(wanted_error, index_error);
    }

}