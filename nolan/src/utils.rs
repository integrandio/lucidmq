use std::path::Path;

pub const LOG_SUFFIX: &str = ".log";
pub const INDEX_SUFFIX: &str = ".index";

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