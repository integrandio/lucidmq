use std::io::Write;
use std::str;

use std::io::prelude::*;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::SeekFrom;
use std::io::ErrorKind;
use serde::{Serialize, Deserialize};

/**
 * Basic data structure that holds our data in our index
 */
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]
pub struct Entry {
    start: u32,
    total: u32
}

pub struct Index {
    //path: String,
    entries: Vec<Entry>,
    index_file: File
}

impl Index {
    /**
     * Create a new index
     */
    pub fn new(index_path: String) -> Self {
        let message = format!("Unable to create and open file {}", index_path);
        let file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .append(true)
            .open(index_path.clone())
            .expect(&message);
        let empty_entry_vec = Vec::new();
        let index = Index {
            //path: index_path,
            entries: empty_entry_vec,
            index_file: file
        };
        return index;
    }

    /**
     * Add a new entry to the index
     */
    pub fn add_entry(&mut self, position: u32, total: u32) {
        let entry = Entry {
            start: position,
            total: total
        };
        //TODO: Error handle this correctly
        let encoded_entry: Vec<u8> = bincode::serialize(&entry).expect("Unable to encode entry");
        self.entries.push(entry);

        let entry_bytes: &[u8] = &encoded_entry[..];
        //TODO: error handle this correctly
        //let written_bytes = 
        self.index_file.write(entry_bytes).expect("Unable to write bytes");
    }

    /**
     * Load the index into memory
     */
    pub fn load_index(&mut self) -> u16{
        //TODO: error handle this correctly
        self.index_file.seek(SeekFrom::Start(0)).expect("Unable to seek to that offset");
        let mut circut_break: bool = false;
        loop {
            let mut buffer = [0; 8];
            //TODO: error handle this correctly
            self.index_file.read_exact(&mut buffer).unwrap_or_else(|error| {
                if error.kind() == ErrorKind::UnexpectedEof {
                    circut_break = true;
                } else {
                    panic!("Unable to read from index file. {:?}", error);
                }
            });
            if circut_break {
                break;
            }
            //TODO: error handle this correctly
            let decoded_entry: Entry = bincode::deserialize(&buffer).expect("unable to deocode buffer");
            self.entries.push(decoded_entry);
        }
        //TODO: error handle this correctly
        return u16::try_from(self.entries.len()).expect("unable to convert to u8.");
    }

    /**
     * Reload any new entries in the index to the entries section of the datastructure
     */
    pub fn reload_index(&mut self) -> u16 {
        let entry_byte_size: u64 = 8;
        let total_current_entires: u64 = self.entries.len() as u64;
        let entry_read_start_bytes: u64 = entry_byte_size * total_current_entires;
        //Start from the last known entry
        self.index_file.seek(SeekFrom::Start(entry_read_start_bytes)).expect("Unable to seek to that offset");
        let mut circut_break: bool = false;
        loop {
            let mut buffer = [0; 8];
            //TODO: error handle this correctly
            self.index_file.read_exact(&mut buffer).unwrap_or_else(|error| {
                if error.kind() == ErrorKind::UnexpectedEof {
                    circut_break = true;
                } else {
                    panic!("Unable to read from index file. {:?}", error);
                }
            });
            if circut_break {
                break;
            }
            //TODO: error handle this correctly
            let decoded_entry: Entry = bincode::deserialize(&buffer).expect("unable to deocode buffer");
            self.entries.push(decoded_entry);
        }
        //TODO: error handle this correctly
        return u16::try_from(self.entries.len()).expect("unable to convert to u8.");
    }

    /**
     * Given an offset, return the entry start
     */
    pub fn return_entry_details_by_offset(&self, offset: usize) -> (u64, usize) {
        // This can throw an exception if the offset is greater than the size of the array, how do we check?
        let entry = self.entries[offset];
        let start_offset: u64 = entry.start.into();
        //TODO: error handle this correctly
        let total_bytes: usize = usize::try_from(entry.total).expect("Unable to convert from u32 to usize");
        return (start_offset, total_bytes)
    }

}
