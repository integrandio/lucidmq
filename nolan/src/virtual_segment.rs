use std::{
    fs::OpenOptions,
    io::{Cursor, Read, Seek, SeekFrom, Write},
    path::Path,
};

use crate::{virtual_index::VirtualIndex, nolan_errors::SegmentError};
use log::{error, info};

pub struct VirtualSegment {
    contents: Cursor<Vec<u8>>,
    position: u32,
    max_bytes: u64,
    pub starting_offset: u16,
    pub next_offset: u16,
    index: VirtualIndex,
    pub full_log_path: String,
}

const LOG_SUFFIX: &str = ".log";
const INDEX_SUFFIX: &str = ".index";

impl VirtualSegment {
    /// Create a virtual segment with the provided starting offset
    pub fn new(base_directory: &str, max_segment_bytes: u64, offset: u16) -> VirtualSegment {
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

    /// Given a directory and the base name of the log and index file, load a new segment into memory.
    pub fn load_segment(
        base_directory: &str,
        segment_offset: u16,
        max_segment_bytes: u64,
    ) -> Result<VirtualSegment, SegmentError> {
        //let segment_error_str = format!("unable to parse base string {segment_offset} into u16");
        // let segment_offset = segment_base.parse::<u16>().map_err(|e| {
        //     error!("{}", e);
        //     SegmentError::new(&segment_error_str)
        // })?;
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
        let computed_size_bytes = u64::try_from(self.contents.get_ref().len() + data.len()) 
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

#[cfg(test)]
mod virtual_segment_tests {
    use std::path::Path;
    use tempdir::TempDir;
    use crate::virtual_segment::VirtualSegment;
    use crate::nolan_errors::SegmentError;

    #[test]
    fn test_one_message() {
        let mut vs = VirtualSegment::new("test_dir", 100, 0);
        let data = "hellos".as_bytes();
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
        let mut vs = VirtualSegment::new("test_dir", 100, 0);
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

    #[test]
    fn test_message_greater_than_segment() {
        let mut vs = VirtualSegment::new("test_dir", 10, 0);
        let data = "000000000000000".as_bytes();
        let segment_error = vs
            .write(data)
            .unwrap_err();
        let want = SegmentError::new("Write not possible. Segment log would be greater than max bytes");

        assert_eq!(want, segment_error);

    }

    #[test]
    fn test_flush() {
        let tmp_dir = TempDir::new("test").expect("Unable to create temp directory");
        let test_dir_path = tmp_dir
            .path()
            .to_str()
            .expect("Unable to convert path to string");
        let mut vs = VirtualSegment::new(test_dir_path, 100, 0);
        let data = "hellos".as_bytes();
        vs
            .write(data)
            .expect("unable to write data to virtual segment");
        vs.flush();

        assert!(Path::new(&vs.full_log_path).exists());
    }
}