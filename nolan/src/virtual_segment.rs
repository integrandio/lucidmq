use std::{
    fs::OpenOptions,
    io::{Cursor, ErrorKind, Read, Seek, SeekFrom, Write},
    path::Path,
};

use crate::{index::Entry, nolan_errors::IndexError, nolan_errors::SegmentError};
use log::{error, info};

pub struct VirtualSegment {
    contents: Cursor<Vec<u8>>,
    position: u32,
    max_bytes: u64,
    starting_offset: u16,
    next_offset: u16,
    index: VirtualIndex,
    full_log_path: String,
}

const LOG_SUFFIX: &str = ".log";
const INDEX_SUFFIX: &str = ".index";

impl VirtualSegment {
    /**
     * Create a virtual segment with the provided starting offset
     */
    pub fn new(base_directory: String, max_segment_bytes: u64, offset: u16) -> VirtualSegment {
        info!("Creating a new virtual segment");
        let log_file_path =
            Self::create_segment_file_name(&base_directory, offset, LOG_SUFFIX);
        let index_file_name =
            Self::create_segment_file_name(&base_directory, offset, INDEX_SUFFIX);
        let new_virtual_index = VirtualIndex::new(index_file_name);
        VirtualSegment {
            contents: Cursor::new(Vec::new()),
            position: 0,
            max_bytes: max_segment_bytes,
            starting_offset: offset,
            next_offset: offset,
            index: new_virtual_index,
            full_log_path: log_file_path,
        }
    }

    /**
     * Given a directory and the base name of the log and index file, load a new
     * segment into memory.
     */
    pub fn load_segment(
        base_directory: &str,
        segment_base: &str,
        max_segment_bytes: u64,
    ) -> Result<VirtualSegment, SegmentError> {
        let segment_offset = segment_base.parse::<u16>().map_err(|e| {
            error!("{}", e);
            SegmentError::new("unable to parse base string into u16")
        })?;
        let log_file_name =
            Self::create_segment_file_name(base_directory, segment_offset, LOG_SUFFIX);

        let mut log_file = OpenOptions::new()
            .create(false)
            .read(true)
            .write(false)
            .append(false)
            .open(log_file_name.clone())
            .map_err(|e| {
                error!("{}", e);
                SegmentError::new("unable to open log file")
            })?;
        
        //Create our buffer
        let mut buffer = Vec::new();
        // Read contents from the file into the buffer
        let total_bytes_read= log_file.read_to_end(&mut buffer).expect("unable to read file to buffer");

        // This would be unnesseary if we used u64 for the position
        let current_segment_postion: u32 = u32::try_from(total_bytes_read).map_err(|e| {
            error!("{}", e);
            SegmentError::new("unable to convert from u64 to u32")
        })?;

        let index_file_name =
            Self::create_segment_file_name(base_directory.clone(), segment_offset, INDEX_SUFFIX);
        let mut loaded_index = VirtualIndex::new(index_file_name);

        let mut total_entries = loaded_index.load_index().map_err(|e| {
            error!("{}", e);
            SegmentError::new("unable to load index")
        })?;
        total_entries += segment_offset;

        let segment = VirtualSegment {
            contents: Cursor::new(buffer),
            position: current_segment_postion,
            max_bytes: max_segment_bytes,
            starting_offset: segment_offset,
            next_offset: total_entries,
            index: loaded_index,
            full_log_path: log_file_name,
        };

        Ok(segment)
    }

    /**
     * Given a byte array, write that data to the corresponding log and index.
     * Return the offset in the segment that was written to.
     */
    pub fn write(&mut self, data: &[u8]) -> Result<u16, SegmentError> {
        let computed_size_bytes = u64::try_from(self.contents.get_ref().len())
            .expect("unable to convert from usize to u64");
        if computed_size_bytes > self.max_bytes {
            return Err(SegmentError::new(
                "Write not possible. Segment log would be greater than max bytes",
            ));
        }
        let u_bytes = self.contents.write(data).map_err(|e| {
            error!("{}", e);
            SegmentError::new("unable to write to log file")
        })?;
        let written_bytes: u32 = u32::try_from(u_bytes).map_err(|e| {
            error!("{}", e);
            SegmentError::new("unable to convert from usize to u32")
        })?;
        self.index
            .add_entry(self.position, written_bytes)
            .map_err(|e| {
                error!("{}", e);
                SegmentError::new("unable to add entry to index")
            })?;
        self.position += written_bytes;
        let offset_written = self.next_offset;
        self.next_offset += 1;
        Ok(offset_written)
    }

    /**
     * Given an offset, find the entry in the index and get the bytes fromt he log
     */
    pub fn read_at(&mut self, offset: usize) -> Result<Vec<u8>, SegmentError> {
        // This condition is only applied when we're dealing with segment 0, can this be combined below??
        if (self.starting_offset == 0 && offset >= usize::from(self.next_offset))
            || (offset >= usize::from(self.next_offset - self.starting_offset))
        {
            return Err(SegmentError::new("offset is out of bounds"));
        }
        let (start, total) = self
            .index
            .return_entry_details_by_offset(offset)
            .map_err(|e| {
                error!("{}", e);
                SegmentError::new("unable to get entry details from index")
            })?;
        // Let's create our buffer
        let mut buffer = vec![0; total];
        // Seek to entries start position
        self.contents.seek(SeekFrom::Start(start)).map_err(|e| {
            error!("{}", e);
            SegmentError::new("unable to seek to offset in the log file")
        })?;
        // Read log file bytes into the buffer
        self.contents.read_exact(&mut buffer).map_err(|e| {
            error!("{}", e);
            SegmentError::new("unable to read into buffer")
        })?;
        Ok(buffer)
    }

    pub fn flush(&self) {
        let mut log_file = OpenOptions::new()
            .create(true)
            .read(false)
            .write(true)
            .append(false)
            .open(self.full_log_path.clone())
            .expect("Unable to create and open file");

        self.index.flush();
        
        log_file
            .write_all(self.contents.get_ref())
            .expect("Unable to flush contents to file");
    }

    /**
     * Given a directory, a starting offset and a file type suffix, create and return the path to the file.
     */
    pub fn create_segment_file_name(
        directory: &str,
        starting_offset: u16,
        suffix: &str,
    ) -> String {
        let file_name = format!("{:0>5}{}", starting_offset, suffix);
        let new_file = Path::new(&directory).join(file_name);
        //TODO: Error handle this
        String::from(new_file.to_str().expect("unable to convert path to string"))
    }
}

struct VirtualIndex {
    contents: Cursor<Vec<u8>>,
    entries: Vec<Entry>,
    full_index_file_path: String,
}

impl VirtualIndex {
    fn new(index_path: String) -> VirtualIndex {
        VirtualIndex {
            contents: Cursor::new(Vec::new()),
            entries: Vec::new(),
            full_index_file_path: index_path,
        }
    }

    /**
     * Add a new entry to the index
     */
    fn add_entry(&mut self, start_position: u32, total_bytes: u32) -> Result<bool, IndexError> {
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
    fn return_entry_details_by_offset(&self, offset: usize) -> Result<(u64, usize), IndexError> {
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
            self.contents.write(&buffer);

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
mod virtual_segment_tests {

    use crate::virtual_segment::VirtualSegment;

    #[test]
    fn test_one_message() {
        let mut vs = VirtualSegment::new(String::from("test_dir"), 100, 0);
        let data = "heellos".as_bytes();
        let offset = vs
            .write(data)
            .expect("unable to write data to virtual segment");

        let retrieve_data = vs
            .read_at(offset.into())
            .expect("Failed to get message from virtual segment");
        assert_eq!(data, &*retrieve_data);
    }

    #[test]
    fn test_multiple_message() {
        let mut vs = VirtualSegment::new(String::from("test_dir"), 100, 0);
        let messages = ["hello", "world", "im", "here"];
        for (i, message) in messages.iter().enumerate() {
            let data = message.as_bytes();
            let offset = vs.write(data).expect("unable to write to virtual seg");
            assert_eq!(i, offset.into());
        }

        for (i, message) in messages.iter().enumerate() {
            let retrieve_data = vs
                .read_at(i)
                .expect("Failed to get message from virtual segment");
            assert_eq!(message.as_bytes(), &*retrieve_data);
        }
    }
}

#[cfg(test)]
mod virtual_index_tests {
    use crate::virtual_segment::VirtualIndex;

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
}
