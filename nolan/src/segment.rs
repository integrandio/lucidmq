use std::io::Write;
use std::str;
use std::io::prelude::*;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::SeekFrom;
use std::path::{Path};

use crate::index;

pub struct Segment {
    pub file_name: String,
    position: u32,
    max_bytes: u64,
    pub starting_offset: u16,
    pub next_offset: u16,
    pub directory: String,
    log_file: File,
    index: index::Index
}

const LOG_SUFFIX: &str = ".log";
const INDEX_SUFFIX: &str = ".index";

impl Segment {
    /**
     * Create a new segment with the provided starting offset
     */
    pub fn new(directory: String, offset: u16) -> Self {
        println!("Creating a new segment");
        let log_file_name = Self::create_segment_file_name(directory.clone(), offset, String::from(LOG_SUFFIX));
       
        let file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .append(true)
            .open(log_file_name.clone())
        .expect("Unable to create and open file");

        let index_file_name = Self::create_segment_file_name(directory.clone(), offset, String::from(INDEX_SUFFIX));
        let index = index::Index::new(index_file_name);

        let segment = Segment {
            file_name: log_file_name,
            position: 0,
            max_bytes: 255,
            starting_offset: offset,
            next_offset: offset,
            directory: directory,
            log_file: file,
            index: index
        };
        return segment;
    }

    /**
     * Given a directory and the base name of the log and index file, load a new
     * segment into memory.
     */
    pub fn load_segment(directory: String, segment_base: String) -> Self {
        let segment_offset = segment_base.parse::<u16>().expect("Unable to parse segment base into int.");
        let log_file_name = Self::create_segment_file_name(directory.clone(), segment_offset, String::from(LOG_SUFFIX));
        
       
        let file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .append(true)
            .open(log_file_name.clone())
        .expect("Unable to create and open file");

        //TODO: Handle error
        let current_segment_postion: u32 = u32::try_from(file.metadata().unwrap().len()).expect("Unable to convert u int type..");

        let index_file_name = Self::create_segment_file_name(directory.clone(), segment_offset, String::from(INDEX_SUFFIX));
        let mut index = index::Index::new(index_file_name);

        let mut total_entries = index.load_index();
        total_entries = total_entries + segment_offset;

        let segment = Segment {
            file_name: log_file_name,
            position: current_segment_postion,
            max_bytes: 255,
            starting_offset: segment_offset,
            next_offset: total_entries,
            directory: directory,
            log_file: file,
            index: index
        };

        return segment;
    }

    /**
     * Reload the segment with the most up to date data from the index.
     */
    pub fn reload(&mut self) {
        // load the entries from the index
        let mut total_entries = self.index.reload_index();
        //Calculate and set the next offset
        total_entries = total_entries + self.starting_offset;
        self.next_offset = total_entries;
    }

    /**
     * Given a directory, a starting offset and a file type suffix, create and return the path to the file.
     */
    pub fn create_segment_file_name(directory: String, starting_offset: u16, suffix: String) -> String {
        let file_name = format!("{:0>5}{}", starting_offset, suffix);
        let new_file = Path::new(&directory).join(file_name);
        //TODO: Error handle this
        return String::from(new_file.to_str().expect("unable to convert path to string")); 
    }

    /**
     * Given a byte array, write that data to the corresponding log and index.
     * TODO: replace result string with a custom error
     */
    pub fn write(&mut self, data: &[u8]) -> Result<bool, String>{
        //TODO: error handle this
        let computed_size_bytes = self.log_file.metadata().expect("unable to get metadata from file").len();
        if computed_size_bytes > self.max_bytes{
            return Err("Write not possible. Segment log would be greater than max bytes".to_string());
        }
        //TODO: error handle this
        let u_bytes = self.log_file.write(data).expect("Unable to write to file");
        //TODO: error handle this
        let written_bytes: u32 = u32::try_from(u_bytes).expect("big problem");
        self.index.add_entry(self.position, written_bytes.clone());
        self.position = self.position + written_bytes;
        self.next_offset= self.next_offset+1;
        return Ok(true);
    }

    /**
     * Given an offset, find the entry in the index and get the bytes fromt he log
     * TODO: replcae result string with custom error
     */
    pub fn read_at(&mut self, offset: usize) -> Result<Vec<u8>, String>{
        // This condition is only applied when we're dealing with segment 0, can this be combined below??
        if self.starting_offset == 0 && offset >= usize::from(self.next_offset){
            println!("Offset {} is out of bounds for file: {}", offset, self.file_name);
            return Err("Offset is out of bounds".to_string());
        } else if offset >= usize::from(self.next_offset - self.starting_offset) {
            println!("Offset {} is out of bounds for file: {}", offset, self.file_name);
            return Err("Offset is out of bounds".to_string());    
        }
        let (start, total) = self.index.return_entry_details_by_offset(offset);
        println!("{:?} {:?}", start, total);
        let mut buffer = vec![0; total];
        // TODO: error handle this better
        self.log_file.seek(SeekFrom::Start(start)).expect("Unable to seek to that offset");
        // TODO: error handle this better
        self.log_file.read_exact(&mut buffer).expect("Unable to read into buffer");
        println!("{:?}", buffer);
        return Ok(buffer);
    }
}