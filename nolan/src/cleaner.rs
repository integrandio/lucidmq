use crate::segment::Segment;

#[derive(Default)]
pub struct Cleaner {
    retention_bytes: u64
}

impl Cleaner {
    /**
     * Initialize a new cleaner, based on retention
     */
    pub fn new(bytes_to_retain: u64) -> Cleaner {
        let cleaner = Cleaner {
            retention_bytes: bytes_to_retain
        };
        return cleaner;
    }

    /**
     * Cleans up the segments based on the cleaners retention bytes
     */
    pub fn clean(&self, segments: &mut Vec<Segment>) {
        let mut total_bytes = 0;
        let mut segment_postion = segments.len();
        for i in (0..segment_postion).rev() {
            if total_bytes > self.retention_bytes {
                break;
            }
            
            let segment = segments.get(i).unwrap();
            total_bytes += segment.position as u64;
            segment_postion -=1;
        }
        for _j in 0..segment_postion {
            let segmet = segments.get_mut(0).unwrap();
            segmet.delete();
            //Remove the first index of the segment
            segments.remove(0);
        }
    }
}
