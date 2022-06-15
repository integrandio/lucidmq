use std::io::Write;
use std::str;

use crate::nolan_errors::IndexError;
use log::error;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::ErrorKind;
use std::io::SeekFrom;

/**
 * Basic data structure that holds our data in our index
 */
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]
pub struct Entry {
    start: u32,
    total: u32,
}

pub struct Index {
    pub file_name: String,
    entries: Vec<Entry>,
    index_file: File,
}

impl Index {
    /**
     * Create a new index
     */
    pub fn new(index_path: String) -> Index {
        let message = format!("Unable to create and open file {}", index_path);
        let file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .append(true)
            .open(index_path.clone())
            .expect(&message);
        let empty_entry_vec = Vec::new();
        Index {
            file_name: index_path,
            entries: empty_entry_vec,
            index_file: file,
        }
    }

    /**
     * Add a new entry to the index
     */
    pub fn add_entry(&mut self, start_position: u32, total_bytes: u32) -> Result<bool, IndexError> {
        let entry = Entry {
            start: start_position,
            total: total_bytes,
        };
        let encoded_entry: Vec<u8> = bincode::serialize(&entry).map_err(|e| {
            error!("{}", e);
            IndexError::new("Unable to serialize entry")
        })?;

        self.entries.push(entry);

        let entry_bytes: &[u8] = &encoded_entry[..];
        self.index_file.write(entry_bytes).map_err(|e| {
            error!("{}", e);
            IndexError::new("Unable to write entry to index file")
        })?;
        Ok(true)
    }

    /**
     * Load the index into memory
     */
    pub fn load_index(&mut self) -> Result<u16, IndexError> {
        self.index_file.seek(SeekFrom::Start(0)).map_err(|e| {
            error!("{}", e);
            IndexError::new("unable seek to begining of the index")
        })?;
        let mut circut_break: bool = false;
        loop {
            let mut buffer = [0; 8];
            //TODO: error handle this correctly
            self.index_file
                .read_exact(&mut buffer)
                .unwrap_or_else(|error| {
                    if error.kind() == ErrorKind::UnexpectedEof {
                        circut_break = true;
                    } else {
                        // error!{"{}", error}
                        // return IndexError::new("unable seek to begining of the index");
                        panic!("{}", error)
                    }
                });
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

    /**
     * Reload any new entries in the index to the entries section of the datastructure
     */
    pub fn reload_index(&mut self) -> Result<u16, IndexError> {
        let entry_byte_size: u64 = 8;
        let total_current_entires: u64 = self.entries.len() as u64;
        let entry_read_start_bytes: u64 = entry_byte_size * total_current_entires;
        //Start from the last known entry
        self.index_file
            .seek(SeekFrom::Start(entry_read_start_bytes))
            .map_err(|e| {
                error!("{}", e);
                IndexError::new("unable to seek to last entry offset")
            })?;
        // Iterate through the file bytes and conver them to entries
        let mut circut_break: bool = false;
        loop {
            let mut buffer = [0; 8];
            //TODO: error handle this correctly
            self.index_file
                .read_exact(&mut buffer)
                .unwrap_or_else(|error| {
                    if error.kind() == ErrorKind::UnexpectedEof {
                        circut_break = true;
                    } else {
                        panic!("Unable to read from index file. {:?}", error);
                    }
                });
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

    /**
     * Given an offset, return the entry start
     */
    pub fn return_entry_details_by_offset(
        &self,
        offset: usize,
    ) -> Result<(u64, usize), IndexError> {
        // This can throw an exception if the offset is greater than the size of the array, how do we check?
        let entry = self.entries[offset];
        let start_offset: u64 = entry.start.into();
        //TODO: error handle this correctly
        let total_bytes: usize = usize::try_from(entry.total).map_err(|e| {
            error!("{}", e);
            IndexError::new("unable to convert from u32 to usize")
        })?;
        Ok((start_offset, total_bytes))
    }

    // /**
    //  * Close the index file
    //  */
    // pub fn close(&self) {
    //     drop(&self.index_file);
    // }
}
