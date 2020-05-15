use std::{fs, path};

pub struct FileReader;

impl FileReader {

    pub fn read(file: &str) -> String {
        match fs::read_to_string(path::Path::new(file)) {
            Ok(val) => val,
            Err(why) => panic!(why.to_string()),
        }
    }
}