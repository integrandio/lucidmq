//! Nolan is a crate that specifies a implemeation for a commitlog storage structure.
use log::{error, info, warn};
use std::fs;

use crate::cleaner::Cleaner;
use crate::nolan_errors::{CommitlogError, SegmentError};
use crate::segment::Segment;
use crate::virtual_segment::VirtualSegment;
use std::collections::HashMap;
use crate::utils;

/// Commitlog is a struct that represents the logs stored on memory and on disc.
/// At it's core, a commitlog is a directory that is made up of segments(log and index files)
/// that are stored within that directory.
pub struct Commitlog {
    directory: String,
    segments: Vec<Segment>,
    cleaner: Cleaner,
    max_segment_size: u64,
    current_segment: VirtualSegment,
}

impl Commitlog {
    /// new creates a new commitlog taking in a base directory(where the segments live),
    /// a max segment size in bytes and a the max bytes the that the cleaner will retain
    /// for the entire commitlog.
    pub fn new(
        base_directory: &str,
        max_segment_size_bytes: u64,
        cleaner_bytes_to_retain: u64,
    ) -> Result<Commitlog, CommitlogError> {
        let vec = Vec::new();
        let new_cleaner = Cleaner::new(cleaner_bytes_to_retain);
        let mut commitlog = Commitlog {
            directory: base_directory.to_string(),
            segments: vec,
            cleaner: new_cleaner,
            max_segment_size: max_segment_size_bytes,
            current_segment: VirtualSegment::new(&base_directory, max_segment_size_bytes, 0), // This is just a placeholder
        };
        fs::create_dir_all(base_directory).map_err(|e| {
            error!("{}", e);
            CommitlogError::new("Unable to create commitlog directory")
        })?;
        commitlog.load_segments()?;
        Ok(commitlog)
    }

    /// Given bytes, append those bytes onto the current segment. If those bytes will not fit onto the segment,
    /// split the segment by creating a new one and append the bytes there.
    pub fn append(&mut self, data: &[u8]) -> Result<u16, CommitlogError> {
        match self.current_segment.write(data) {
            Ok(segment_offset_written) => {
                info!("Successfully wrote to segment");
                let commitlog_written_offset =
                    self.current_segment.starting_offset + segment_offset_written;
                self.clean().map_err(|e| {
                    error!("{}", e);
                    CommitlogError::new("Unable to clean commitlog")
                })?;
                Ok(commitlog_written_offset)
            }
            Err(err) => {
                let split_err = SegmentError::new(
                    "Write not possible. Segment log would be greater than max bytes",
                );
                if err == split_err {
                    self.split().expect("Unable to split commitlog");
                    self.append(data)
                } else {
                    Err(CommitlogError::new("Unknown error when writing occured"))
                }
            }
        }
    }

    /// Look through the directory of the commitlog and load the segments into memory.
    /// Also performs some cleanup on non-matching logs and indexes(for example, if there is a log file with a non-matching
    /// index or vice versa)
    fn load_segments(&mut self) -> Result<(), CommitlogError> {
        //let mut files_to_clean: HashMap<String, String> = HashMap::new();
        //let paths = fs::read_dir(&self.directory).expect("Unable to read files in directory.");
        let mut valid_segment_files: Vec<String> = Vec::new();
        let mut files_to_clean: Vec<String> = Vec::new();
        if let Ok(entries) = fs::read_dir(&self.directory) {
            for entry in entries.flatten() {
                //if let Ok(entry) = entry {
                let path = entry.path();
                if let Some(extension) = path.extension() {
                    if extension == utils::LOG_EXTENSION {
                        let mut corresponding_index_path = entry.path();
                        corresponding_index_path.set_extension(utils::INDEX_EXTENSION);
                        if !corresponding_index_path.is_file() {
                            files_to_clean.push(path.to_str().unwrap().into());
                        }
                    } else if extension == utils::INDEX_EXTENSION {
                        let mut corresponding_log_path = entry.path();
                        corresponding_log_path.set_extension(utils::LOG_EXTENSION);
                        if corresponding_log_path.is_file() {
                            if let Some(file_stem) = path.file_stem() {
                                valid_segment_files.push(file_stem.to_str().unwrap().into());
                            }
                        } else {
                            files_to_clean.push(path.to_str().unwrap().into());
                        }
                    }
                }
                //}
            }
        }
        for segment_file in valid_segment_files {
            let loaded_segment = Segment::load_segment(&self.directory, segment_file).map_err(|e| {
                error!("{}", e);
                CommitlogError::new("unable to load segment")
            })?;
            self.segments.push(loaded_segment);
        }

        let mut latest_segment_index = self.segments.len();
        if latest_segment_index == 0 {
            return Ok(());
        } else {
            self.segments
                .sort_by(|a, b| a.starting_offset.cmp(&b.starting_offset));
            latest_segment_index -= 1;
            if latest_segment_index > 0 {
                let current_segment_from_loaded_segments = self
                    .segments
                    .get(latest_segment_index)
                    .expect("Unable to set current segment from loaded segments");
                self.current_segment = VirtualSegment::load_segment(
                    &self.directory,
                    current_segment_from_loaded_segments.starting_offset,
                    self.max_segment_size,
                )
                .expect("unable to load virtual segment");
            }
        }

        for file_to_clean in files_to_clean {
            fs::remove_file(file_to_clean).map_err(|e| {
                error!("{}", e);
                CommitlogError::new("unable to remove files during cleanup")
            })?;
        }
        Ok(())
    }

    // /**
    //  * Update the most current segment with information.
    //  */
    // fn reload_current_segment(&mut self) {
    //     //What do we want to do when reload is called??
    //     if self.segments.is_empty() {
    //         return;
    //     }
    //     let index = self.current_segment_index.get_mut();
    //     // Properly error handle this
    //     let current_segment = self
    //         .segments
    //         .get_mut(*index)
    //         .expect("Unable to get current segment");
    //     current_segment.reload().expect("Unable to reload segment");
    // }

    /// Get the max segment size that is assigned to the commitlog.
    pub fn get_max_segment_size(&self) -> u64 {
        self.max_segment_size
    }

    /// Load segments in from the commitlog directory that have not been loaded into memory yet.
    pub fn reload_segments(&mut self) {
        //self.reload_current_segment();
        let mut segment_map: HashMap<String, String> = HashMap::new();
        let mut valid_segments_found: Vec<String> = Vec::new();
        if let Ok(entries) = fs::read_dir(&self.directory) {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(extension) = path.extension() {
                    let file_stem = path
                        .file_stem()
                        .expect("Unable to get file stem")
                        .to_str()
                        .unwrap();
                    //let base_file_name = path.to_str().unwrap();
                    if extension == utils::LOG_EXTENSION {
                        if segment_map.contains_key(file_stem) {
                            valid_segments_found.push(file_stem.into())
                        } else {
                            segment_map.insert(file_stem.into(), utils::LOG_EXTENSION.to_string());
                        }
                    } else if extension == utils::INDEX_EXTENSION {
                        if segment_map.contains_key(file_stem) {
                            valid_segments_found.push(file_stem.into())
                        } else {
                            segment_map.insert(file_stem.into(), utils::INDEX_EXTENSION.to_string());
                        }
                    } else {
                        warn!("extension not found {:?}", extension);
                    }
                }
            }
        }

        let segments_to_add: Vec<String> = valid_segments_found
            .into_iter()
            .filter(|segment| {
                let segment_offset = segment
                    .clone()
                    .parse::<u16>()
                    .expect("Unable to parse segment base into int.");
                let mut segment_exists = false;
                for existing_segment in &self.segments {
                    if segment_offset == existing_segment.starting_offset {
                        segment_exists = true;
                        break;
                    }
                }
                !segment_exists
            })
            .collect();

        if segments_to_add.is_empty() {
            return;
        }

        for segment in segments_to_add {
            info!("updating a new segment {}", segment);
            let loaded_segment = Segment::load_segment(&self.directory, segment)
                .expect("unable to laod segment");
            self.segments.push(loaded_segment);
        }

        // let mut latest_segment_index = self.segments.len();
        // if latest_segment_index != 0 {
        //     self.segments
        //         .sort_by(|a, b| a.starting_offset.cmp(&b.starting_offset));
        //     latest_segment_index -= 1;
        //     if latest_segment_index > 0 {
        //         self.current_segment_index = AtomicUsize::new(latest_segment_index);
        //     }
        // }
    }

    /// Create a new segment and set the latest segment value to that new segment. 
    /// Also flushes the sement to the disk, before creating and updating the segment
    fn split(&mut self) -> Result<(), CommitlogError> {
        info!("Spliting commitlog segment");
        //Flush the current virtual segment to disk
        self.current_segment
            .flush()
            .map_err(|e| {
                error!("{}", e);
                CommitlogError::new("Unable to flush commitlog to disk")
            })?;
        // Get the next offset from current segment and create a new segment with it
        let next_offset = self.current_segment.starting_offset + self.current_segment.next_offset;
        self.current_segment =
            VirtualSegment::new(&self.directory, self.max_segment_size, next_offset);
        // Since our last current segment got flushed to disk, reload from disk to update the segments
        self.reload_segments();
        Ok(())
    }

    /// Given an offset, find and read the value from the commitlog for the segment that it is located in.
    pub fn read(&mut self, offset: usize) -> Result<Vec<u8>, CommitlogError> {
        //First check the current segment
        if usize::from(self.current_segment.starting_offset) <= offset {
            let search_offset = offset - usize::from(self.current_segment.starting_offset);
            return match self.current_segment.read_at(search_offset) {
                Ok(buffer) => Ok(buffer),
                Err(err) => {
                    let out_of_bounds = SegmentError::new("offset is out of bounds");
                    if err == out_of_bounds {
                        return Err(CommitlogError::new("Offset does not exist in the commitlog"));
                    } else {
                        return Err(CommitlogError::new("Unexpected error when reading commitlog"))
                    }
                }
            };
        }

        // Check the segments on disk
        let mut segment_index: Option<usize> = None;
        for (i, segment) in self.segments.iter().enumerate() {
            if usize::from(segment.starting_offset) <= offset {
                segment_index = Some(i);
            } else {
                break;
            }
        }

        if let Some(value) = segment_index {
            let segment = self
                .segments
                .get_mut(value)
                .expect("Unable to get current segment");
            let search_offset = offset - usize::from(segment.starting_offset);

            match segment.read_at(search_offset) {
                Ok(buffer) => Ok(buffer),
                Err(err) => {
                    let out_of_bounds = SegmentError::new("offset is out of bounds");
                    if err == out_of_bounds {
                        return Err(CommitlogError::new("Offset does not exist in the commitlog"));
                    } else {
                        return Err(CommitlogError::new("Unexpected error when reading commitlog"))
                    }
                }
            }
        } else {
            error!("offset {} does not exist in the commtlog", offset);
            Err(CommitlogError::new("Offset does not exist in the commitlog"))
        }
    }

    /// Clean calls the cleaner to clean up the commitlog directory, it then updates the latest segment pointer.
    fn clean(&mut self) -> Result<(), CommitlogError>{
        info!("attempting to clean commitlog");
        let cleaner_response = self.cleaner.clean(&mut self.segments);
        let _cleaner_response = match cleaner_response {
            Ok(_res) => info!("Cleaned commitlog successfully."),
            Err(error) => { 
                error!("{}", error);
                return Err(CommitlogError::new("Unbale to clean the commitlog"));
            },
        };
        Ok(())
        // let latest_segment_index = self.segments.len() - 1;
        // self.current_segment_index = AtomicUsize::new(latest_segment_index);
    }

    /// Returns the first offset of the oldest segment stored in the commitlog.
    pub fn get_oldest_offset(&self) -> usize {
        // If there is no segments intialized, just return 0
        if self.segments.len() == 0 {
            return 0;
        }
        let oldest_segment = &self.segments[0];
        let offset = oldest_segment.starting_offset;
        usize::from(offset)
    }

    /// Returns the first offset of the first segment.
    pub fn get_latest_offset(&self) -> usize {
        let offset: u16 = self.current_segment.next_offset;
        usize::from(offset)
    }
}

#[cfg(test)]
mod commitlog_tests {
    use crate::{commitlog::Commitlog, CommitlogError};
    use std::path::Path;
    use tempdir::TempDir;

    #[test]
    fn test_new_commitlog() {
        let tmp_dir = TempDir::new("test").expect("Unable to create temp directory");
        let tmp_dir_string = tmp_dir
            .path()
            .to_str()
            .expect("Unable to conver path to string");
        let commitlog = Commitlog::new(tmp_dir_string, 100, 1000).expect("Unable to create commitlog");
        assert!(Path::new(&commitlog.directory).is_dir());
    }

    #[test]
    fn test_append() {
        let tmp_dir = TempDir::new("test").expect("Unable to create temp directory");
        let tmp_dir_path = tmp_dir
            .path()
            .to_str()
            .expect("Unable to conver path to string");
        let mut cl =
            Commitlog::new(tmp_dir_path, 100, 1000).expect("Unable to create commitlog");
        let test_data = "producer1".as_bytes();
        cl.append(test_data).expect("Unable to append message");

        let retrived_message = cl.read(0).expect("Unable to retrieve message");
        assert_eq!(test_data, &*retrived_message);
    }

    #[test]
    fn test_append_multiple() {
        let tmp_dir = TempDir::new("test").expect("Unable to create temp directory");
        let tmp_dir_path = tmp_dir
            .path()
            .to_str()
            .expect("Unable to conver path to string");
        let mut cl = Commitlog::new(tmp_dir_path, 1000, 10000).expect("Unable to create commitlog");
        let test_data = "message".as_bytes();
        for n in 0..10 {
            let offset = cl.append(test_data).expect("Unable to append message");
            assert_eq!(n, offset);
        }
    }

    #[test]
    fn test_split() {
        let number_of_iterations = 20;
        let tmp_dir = TempDir::new("test").expect("Unable to create temp directory");
        let tmp_dir_path = tmp_dir
            .path()
            .to_str()
            .expect("Unable to conver path to string");
        let mut cl = Commitlog::new(tmp_dir_path, 100, 1000).expect("Unable to create commitlog");

        for i in 0..number_of_iterations {
            let string_message = format!("myTestMessage{}", i);
            let test_data = string_message.as_bytes();
            let offset = cl.append(test_data).expect("Unable to append message");
            println!("{}", offset);
            assert_eq!(offset, i);
        }

        for i in 0..number_of_iterations {
            let string_message = format!("myTestMessage{}", i);
            let test_data = string_message.as_bytes();
            let retrived_message = cl.read(i.into()).expect("Unable to retrieve message");
            assert_eq!(test_data, &*retrived_message);
        }
    }

    #[test]
    fn get_oldest_offset_test() {
        let number_of_iterations = 5;
        let tmp_dir = TempDir::new("test").expect("Unable to create temp directory");
        let tmp_dir_path = tmp_dir
            .path()
            .to_str()
            .expect("Unable to conver path to string");
        let mut cl = Commitlog::new(tmp_dir_path, 1000, 10000).expect("Unable to create commitlog");

        for i in 0..number_of_iterations {
            let string_message = format!("myTestMessage{}", i);
            let test_data = string_message.as_bytes();
            cl.append(test_data).expect("Unable to append message");
        }

        let latest_cl_offset = cl.get_oldest_offset();
        assert_eq!(0, latest_cl_offset);
    }

    #[test]
    fn get_latest_offset_test() {
        let number_of_iterations = 5;
        let tmp_dir = TempDir::new("test").expect("Unable to create temp directory");
        let tmp_dir_path = tmp_dir
            .path()
            .to_str()
            .expect("Unable to conver path to string");
        let mut cl = Commitlog::new(tmp_dir_path, 1000, 10000).expect("Unable to create commitlog");

        for i in 0..number_of_iterations {
            let string_message = format!("myTestMessage{}", i);
            let test_data = string_message.as_bytes();
            cl.append(test_data).expect("Unable to append message");
        }

        let latest_cl_offset = cl.get_latest_offset();
        assert_eq!(number_of_iterations, latest_cl_offset);
    }

    #[test]
    fn test_append_message_bigger_than_segment() {
        let tmp_dir = TempDir::new("test").expect("Unable to create temp directory");
        let tmp_dir_path = tmp_dir
            .path()
            .to_str()
            .expect("Unable to conver path to string");
        let mut cl =
            Commitlog::new(tmp_dir_path, 10, 100).expect("Unable to create commitlog");
        let bytes: [u8; 11] = [0; 11];
        let commitlog_error = cl.append(&bytes).unwrap_err();
        let wanted_error =
            CommitlogError::new("Unknown error when writing occured");
        assert_eq!(wanted_error, commitlog_error);
    }
}
