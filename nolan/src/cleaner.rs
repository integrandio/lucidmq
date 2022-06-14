use log::{error};
use crate::segment::Segment;
use crate::nolan_errors::CleanerError;

#[derive(Default)]
pub struct Cleaner {
    retention_bytes: u64,
}

impl Cleaner {
    /**
     * Initialize a new cleaner, based on retention
     */
    pub fn new(bytes_to_retain: u64) -> Cleaner {
        let cleaner = Cleaner {
            retention_bytes: bytes_to_retain,
        };
        return cleaner;
    }

    /**
     * Cleans up the segments based on the cleaners retention bytes
     */
    pub fn clean(&self, segments: &mut Vec<Segment>) -> Result<bool, CleanerError> {
        let mut total_bytes = 0;
        let mut segment_postion = segments.len();
        for i in (0..segment_postion).rev() {
            if total_bytes > self.retention_bytes {
                break;
            }
            //TODO: error handle this correctly.
            let segment = segments.get(i).expect("Unable to get index");
            total_bytes += segment.position as u64;
            segment_postion -= 1;
        }
        for _j in 0..segment_postion {
            //TODO: error handle this correctly
            let segmet = segments.get_mut(0).expect("Unable to get index");
            segmet.delete().map_err(|e| {
                error!("{}", e);
                return CleanerError::new("unable to delete segment");
            })?;
            //Remove the first index of the segment
            segments.remove(0);
        }
        Ok(true)
    }
}
