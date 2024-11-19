use crate::nolan_errors::CleanerError;
use crate::segment::Segment;
use log::error;

#[derive(Default)]
pub struct Cleaner {
    retention_bytes: u64,
}

impl Cleaner {
    /// Initialize a new cleaner, based on a retention size in bytes
    pub fn new(bytes_to_retain: u64) -> Cleaner {
        Cleaner {
            retention_bytes: bytes_to_retain,
        }
    }

    /// Cleans up the segments based on the cleaners retention bytes. 
    /// If the total bytes stored on segments exceed the max bytes, segments will be removed.
    pub fn clean(&self, segments: &mut Vec<Segment>) -> Result<bool, CleanerError> {
        let mut total_bytes = 0;
        let mut segment_postion = segments.len();
        for i in (0..segment_postion).rev() {
            if total_bytes > self.retention_bytes {
                break;
            }
            match segments.get(i) {
                Some(segment) => {
                    total_bytes += segment.position as u64;
                    segment_postion -= 1;
                },
                None => return Err(CleanerError::new("Unable to get index of the segment"))
            }
        }
        for _j in 0..segment_postion {
            match segments.get_mut(0) {
                Some(segment) => {
                    segment.delete().map_err(|e| {
                        error!("{}", e);
                        CleanerError::new("unable to delete segment")
                    })?;
                    //Remove the first index of the segment
                    segments.remove(0);
                },
                None => return Err(CleanerError::new("Unable to get segment for deletion"))
            }
        }
        Ok(true)
    }
}
