use log::{error, info};
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::io::Write;
use std::path::Path;
use std::str;

use crate::index::Index;
use crate::nolan_errors::SegmentError;

pub struct Segment {
    pub file_name: String,
    pub position: u32,
    max_bytes: u64,
    pub starting_offset: u16,
    pub next_offset: u16,
    pub directory: String,
    log_file: File,
    index: Index,
}

const LOG_SUFFIX: &str = ".log";
const INDEX_SUFFIX: &str = ".index";

impl Segment {
    /**
     * Create a new segment with the provided starting offset
     */
    pub fn new(base_directory: String, max_segment_bytes: u64, offset: u16) -> Segment {
        info!("Creating a new segment");
        let log_file_name = Self::create_segment_file_name(
            base_directory.clone(),
            offset,
            String::from(LOG_SUFFIX),
        );
        let file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .append(true)
            .open(log_file_name.clone())
            .expect("Unable to create and open file");

        let index_file_name = Self::create_segment_file_name(
            base_directory.clone(),
            offset,
            String::from(INDEX_SUFFIX),
        );
        let new_index = Index::new(index_file_name);

        Segment {
            file_name: log_file_name,
            position: 0,
            max_bytes: max_segment_bytes,
            starting_offset: offset,
            next_offset: offset,
            directory: base_directory,
            log_file: file,
            index: new_index,
        }
    }

    /**
     * Given a directory and the base name of the log and index file, load a new
     * segment into memory.
     */
    pub fn load_segment(
        base_directory: String,
        segment_base: String,
    ) -> Result<Segment, SegmentError> {
        let segment_offset = segment_base.parse::<u16>().map_err(|e| {
            error!("{}", e);
            SegmentError::new("unable to parse base string into u16")
        })?;
        let log_file_name = Self::create_segment_file_name(
            base_directory.clone(),
            segment_offset,
            String::from(LOG_SUFFIX),
        );

        let file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .append(true)
            .open(log_file_name.clone())
            .map_err(|e| {
                error!("{}", e);
                SegmentError::new("unable to open log file")
            })?;

        let metadata = file.metadata().map_err(|e| {
            error!("{}", e);
            SegmentError::new("unable get metadata for log file")
        })?;
        // This would be unnesseary if we used u64 for the position
        let current_segment_postion: u32 = u32::try_from(metadata.len()).map_err(|e| {
            error!("{}", e);
            SegmentError::new("unable to convert from u64 to u32")
        })?;

        let index_file_name = Self::create_segment_file_name(
            base_directory.clone(),
            segment_offset,
            String::from(INDEX_SUFFIX),
        );
        let mut loaded_index = Index::new(index_file_name);

        let mut total_entries = loaded_index.load_index().map_err(|e| {
            error!("{}", e);
            SegmentError::new("unable to load index")
        })?;
        total_entries += segment_offset;

        let segment = Segment {
            file_name: log_file_name,
            position: current_segment_postion,
            max_bytes: 255,
            starting_offset: segment_offset,
            next_offset: total_entries,
            directory: base_directory,
            log_file: file,
            index: loaded_index,
        };

        Ok(segment)
    }

    /**
     * Reload the segment with the most up to date data from the index.
     */
    pub fn reload(&mut self) -> Result<bool, SegmentError> {
        // load the entries from the index
        let mut total_entries = self.index.reload_index().map_err(|e| {
            error!("{}", e);
            SegmentError::new("unable to reload index")
        })?;
        //Calculate and set the next offset
        total_entries += self.starting_offset;
        self.next_offset = total_entries;
        Ok(true)
    }

    /**
     * Given a byte array, write that data to the corresponding log and index.
     */
    pub fn write(&mut self, data: &[u8]) -> Result<bool, SegmentError> {
        let computed_size_bytes = self
            .log_file
            .metadata()
            .map_err(|e| {
                error!("{}", e);
                SegmentError::new("unable to reload index")
            })?
            .len();
        if computed_size_bytes > self.max_bytes {
            return Err(SegmentError::new(
                "Write not possible. Segment log would be greater than max bytes",
            ));
        }
        let u_bytes = self.log_file.write(data).map_err(|e| {
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
        self.next_offset += 1;
        Ok(true)
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
        self.log_file.seek(SeekFrom::Start(start)).map_err(|e| {
            error!("{}", e);
            SegmentError::new("unable to seek to offset in the log file")
        })?;
        // Read log file bytes into the buffer
        self.log_file.read_exact(&mut buffer).map_err(|e| {
            error!("{}", e);
            SegmentError::new("unable to read into buffer")
        })?;
        Ok(buffer)
    }

    /**
     * Close the log file and the index file, then delete both of these files.
     */
    pub fn delete(&self) -> Result<bool, SegmentError> {
        //self.close();
        fs::remove_file(&self.file_name).map_err(|e| {
            error!("{}", e);
            SegmentError::new("unable to delete log file")
        })?;
        fs::remove_file(&self.index.file_name).map_err(|e| {
            error!("{}", e);
            SegmentError::new("unable to delete index file")
        })?;
        Ok(true)
    }

    /**
     * Given a directory, a starting offset and a file type suffix, create and return the path to the file.
     */
    pub fn create_segment_file_name(
        directory: String,
        starting_offset: u16,
        suffix: String,
    ) -> String {
        let file_name = format!("{:0>5}{}", starting_offset, suffix);
        let new_file = Path::new(&directory).join(file_name);
        //TODO: Error handle this
        String::from(new_file.to_str().expect("unable to convert path to string"))
    }
}

// #[cfg(test)]
// mod segment_tests {
//     use std::path::Path;

//     use crate::segment::Segment;

//     #[test]
//     fn test_new_segment() {
//         let test_dir_path = String::from("test");
//         Segment::new(test_dir_path.clone(), 100, 1000);
//         //Check if the directory exists
//         assert!(Path::new(&test_dir_path).is_dir());
//         //Check if the segment file and index file exists
//     }
// }
