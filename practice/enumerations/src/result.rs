use std::fs;
use std::io;
pub fn read_file(path: &str) -> Option<String> {
    let file_result = fs::read_to_string(path);
    match file_result {
        Ok(file_content) => {
            println!("file read successfuly {:?}", file_content);
            Some(file_content)
        }
        Err(error) => {
            println!("error in reading the file {:?}", error);
            None
        }
    }
}
