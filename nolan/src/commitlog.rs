use core::panic;
use std::fs;
use std::sync::atomic::AtomicUsize;
//use std::sync::{Arc, Mutex};
use crate::segment::{Segment};
use crate::cleaner::{Cleaner};
use std::collections::HashMap;

#[derive(Default)]
pub struct Commitlog {
    directory: String,
    segments: Vec<Segment>,
    cleaner: Cleaner,
    current_segment_index: AtomicUsize
    //current_segment: Arc<Mutex<usize>>
    //current_segment: Option<Arc<Mutex<Segment>>>
}

impl Commitlog {
    /**
     * new creates a new commitlog
     */
    pub fn new(path: String) -> Commitlog {
        let vec = Vec::new();
        let cleaner = Cleaner::new(1000);
        let mut clog = Commitlog {
            directory: path.clone(),
            segments: vec,
            cleaner: cleaner,
            ..Default::default()
        };
        //TODO: error handle this
        fs::create_dir_all(path).expect("Unable to create directory");
        clog.load_segments();
        return clog;
    }

    /**
     * Given a byte array, attempt to add to our commitlog on the latest segment
     */
    pub fn append(&mut self, data: &[u8]) {
        let total_segment = self.segments.len();
        if total_segment == 0 {
            let segment = Segment::new(self.directory.clone(), 0);
            //self.current_segment = Some(Arc::new(Mutex::new(&segment)));
            self.segments.push(segment);
            self.current_segment_index = AtomicUsize::new(0);
        }
        let index = self.current_segment_index.get_mut();
        // Properly error handle this
        let current_segment = self.segments.get_mut(*index).expect("Unable to get current segment");
        match current_segment.write(data){
            Ok(_thing) => {
                println!("Successfully wrote to segment");
                self.clean();
            },
            Err(err) => {
                if err == "Write not possible. Segment log would be greater than max bytes" {
                    self.split();
                    self.append(data);
                    return;
                }
                panic!("Another error showed up from here!!")

            }
        }
    }

    /**
     * Look through the directory of the commitlog and load the segments into memory.
     * Also performs some cleanup on non-matching logs and indexes
     */
    fn load_segments(&mut self) {
        //let mut files_to_clean: HashMap<String, String> = HashMap::new();
        //let paths = fs::read_dir(&self.directory).expect("Unable to read files in directory.");
        let mut valid_segment_files: Vec<String> = Vec::new();
        let mut files_to_clean: Vec<String> = Vec::new();
        if let Ok(entries) = fs::read_dir(&self.directory) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if let Some(extension) = path.extension() {
                        if extension == "log" {
                            let mut corresponding_index_path = entry.path();
                            corresponding_index_path.set_extension("index");
                            if !corresponding_index_path.is_file() {
                                files_to_clean.push(path.to_str().unwrap().into());
                            }
                        } else if extension == "index" {
                            let mut corresponding_log_path = entry.path();
                            corresponding_log_path.set_extension("log");
                            if corresponding_log_path.is_file() {
                                if let Some(file_stem) = path.file_stem() {
                                    valid_segment_files.push(file_stem.to_str().unwrap().into());
                                }
                            } else {
                                files_to_clean.push(path.to_str().unwrap().into());
                            }
                        }
                    }
                }
            }
        }
        for segment_file in valid_segment_files {
            let loaded_segment = Segment::load_segment(self.directory.clone(), segment_file);
            self.segments.push(loaded_segment);
        }

        let mut latest_segment_index = self.segments.len();
        if latest_segment_index == 0 {
            return;
        } else {
            self.segments.sort_by(|a, b| a.starting_offset.cmp(&b.starting_offset));
            latest_segment_index = latest_segment_index - 1;
            if latest_segment_index > 0 {
                self.current_segment_index = AtomicUsize::new(latest_segment_index);
            }
        }

        for file_to_clean in files_to_clean {
            fs::remove_file(file_to_clean).expect("Unable to delete file");
        }
    }

    /**
     * Update the most current segment with information.
     */
    fn reload_current_segment(&mut self) {
        let index = self.current_segment_index.get_mut();
        // Properly error handle this
        let current_segment = self.segments.get_mut(*index).expect("Unable to get current segment");
        current_segment.reload()
    }

    pub fn reload_segments(&mut self) {
        self.reload_current_segment();
        let mut file_map: HashMap<String, String> = HashMap::new();
        let mut new_segments: Vec<String> = Vec::new();
        if let Ok(entries) = fs::read_dir(&self.directory) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if let Some(extension) = path.extension() {
                        let file_stem = path.file_stem().expect("Unable to get file stem").to_str().unwrap();
                        //let base_file_name = path.to_str().unwrap();
                        if extension == "log" {                   
                            if file_map.contains_key(file_stem) {
                                new_segments.push(file_stem.into())
                            } else {
                                file_map.insert(file_stem.into(), "log".to_string());
                            }
                        } else if extension == "index" {
                            if file_map.contains_key(file_stem) {
                                new_segments.push(file_stem.into())
                            } else {
                                file_map.insert(file_stem.into(), "index".to_string());
                            }
                        } else {
                            println!("extension not found {:?}", extension);
                        }
                    }
                }
            }
        }

        let mut new_segments_thing: Vec<String> = Vec::new();
        for segment in new_segments {
            let segment_offset = segment.clone().parse::<u16>().expect("Unable to parse segment base into int.");
            let mut segment_exists = false;
            for existing_segment in &self.segments {
                if segment_offset == existing_segment.starting_offset {
                    segment_exists = true;
                    break;
                }
            }
            if !segment_exists {
                new_segments_thing.push(segment.clone());
            }
        }

        if new_segments_thing.is_empty() {
            return
        }

        for segment in new_segments_thing {
            println!("Adding new segment {}", segment);
            let loaded_segment = Segment::load_segment(self.directory.clone(), segment);
            self.segments.push(loaded_segment);
        }

        let mut latest_segment_index = self.segments.len();
        if latest_segment_index == 0 {
            return;
        } else {
            self.segments.sort_by(|a, b| a.starting_offset.cmp(&b.starting_offset));
            latest_segment_index = latest_segment_index - 1;
            if latest_segment_index > 0 {
                self.current_segment_index = AtomicUsize::new(latest_segment_index);
            }
        }
    }

    /**
     * Create a new segment and set the latest segment value to that new segment
     */
    fn split(&mut self) {
        // Get the current segment
        let index = self.current_segment_index.get_mut();
        let current_segment = self.segments.get_mut(*index).expect("Unable to get current segment");

        // Get the next offset from current segment and create a new segment with it
        let next_offset= current_segment.next_offset;
        let segment = Segment::new(self.directory.clone(), next_offset);

        // Update our object with our new segment and details.
        self.segments.push(segment);
        let latest_segment_index = self.segments.len() - 1;
        self.current_segment_index = AtomicUsize::new(latest_segment_index);
    }

    /**
     * Given an offset, find and read the value from the commitlog for the 
     * segment that it is located in.
     */
    pub fn read(&mut self, offset: usize) -> Result<Vec<u8>, String> {
        let mut segment_index: Option<usize> = None;
        for (i, segment) in self.segments.iter().enumerate() {
            if usize::from(segment.starting_offset) <= offset {
                segment_index = Some(i);
            } else {
                break;
            }
        }

        if let Some(value) = segment_index {
            let segment = self.segments.get_mut(value).expect("Unable to get current segment");
            let search_offset = offset - usize::from(segment.starting_offset);
            
            match segment.read_at(search_offset) {
                Ok(buffer) => {
                    Ok(buffer)
                }
                Err(err) => {
                    if err == "Offset is out of bounds" {
                        return Err("Offset does not exist in the commitlog".to_string());
                    } else {
                        panic!("unexpected error reached")
                    }
                }
            } 
        } else {
            //return error
            println!("offset {} does not exist in the commtlog", offset);
            return Err("Offset does not exist in the commitlog".to_string());
        }
    }

    pub fn clean(&mut self) {
        self.cleaner.clean(&mut self.segments);
    }
}

#[cfg(test)]
mod commitlog_tests {
    use std::path::Path;

    use crate::commitlog::Commitlog;

    #[test]
    fn it_works() {
        let test_dir_path = String::from("test");
        Commitlog::new(test_dir_path.clone());
        assert!(Path::new(&test_dir_path).is_dir());
    }
}