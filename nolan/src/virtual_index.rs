use std::{
    fs::OpenOptions,
    io::{Cursor, ErrorKind, Read, Seek, SeekFrom, Write},
};

use crate::{index::Entry, nolan_errors::IndexError};
use log::error;

pub struct VirtualIndex {
    contents: Cursor<Vec<u8>>,
    entries: Vec<Entry>,
    full_index_file_path: String,
}

impl VirtualIndex {
    pub fn new(index_path: String) -> VirtualIndex {
        VirtualIndex {
            contents: Cursor::new(Vec::new()),
            entries: Vec::new(),
            full_index_file_path: index_path,
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
        self.contents.write(entry_bytes).map_err(|e| {
            error!("{}", e);
            IndexError::new("Unable to write entry to index file")
        })?;
        Ok(true)
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

    /// Writes the buffered data to disk.
    pub fn flush(&self) {
        let mut index_file = OpenOptions::new()
            .create(true)
            .read(false)
            .write(true)
            .append(false)
            .open(self.full_index_file_path.clone())
            .expect("Unable to create and open file");

        index_file
            .write_all(self.contents.get_ref())
            .expect("Unable to flush contents to file");
    }

    /**
     * Load the index into memory
     */
    pub fn load_index(&mut self) -> Result<u16, IndexError> {
        let mut index_file = OpenOptions::new()
            .create(false)
            .read(true)
            .write(false)
            .append(false)
            .open(self.full_index_file_path.clone())
            .expect("Unable to create and open file");

        index_file.seek(SeekFrom::Start(0)).map_err(|e| {
            error!("{}", e);
            IndexError::new("unable seek to begining of the index")
        })?;
        let mut circut_break: bool = false;
        loop {
            let mut buffer = [0; 8];
            //TODO: error handle this correctly
            index_file.read_exact(&mut buffer).unwrap_or_else(|error| {
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
            //First write bytes to our virtual buffer
            self.contents
                .write(&buffer)
                .expect("unable to write to buffer");

            //Decode the entry and add it to our entry vector
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
}

#[cfg(test)]
mod virtual_index_tests {
    use crate::virtual_index::VirtualIndex;
    #[test]
    fn test_add_entry() {
        let mut test_index = VirtualIndex::new(String::from("test_dir/test.index"));

        let start_position = 0;
        let total_bytes = 10;
        test_index
            .add_entry(start_position, total_bytes)
            .expect("Unable to add entry");

        let retrieved_entry = test_index.entries.get(0).expect("Got entry");
        assert_eq!(start_position, retrieved_entry.start);
        assert_eq!(total_bytes, retrieved_entry.total);
    }

    #[test]
    fn test_add_multiple_entry() {
        let mut test_index = VirtualIndex::new(String::from("test_dir/test.index"));

        let mut start_position = 0;
        let total_bytes = 10;

        for _ in 0..9 {
            test_index
                .add_entry(start_position, total_bytes)
                .expect("Unable to add entry");
            start_position += total_bytes
        }

        start_position = 0;
        for i in 0..9 {
            let retrieved_entry = test_index.entries.get(i).expect("Got entry");
            assert_eq!(start_position, retrieved_entry.start);
            assert_eq!(total_bytes, retrieved_entry.total);
            start_position += total_bytes
        }
    }
}
