use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

pub fn read<'a>(input: &'a mut String, path: &'a String, file_name: String) -> Vec<&'a str> {
    let file_path = format!("{}{}", path.to_string(), file_name.to_string());
    let file_path = PathBuf::from(file_path);

    let mut file = File::open(file_path).expect("File not found");
    file.read_to_string(input)
        .expect("Error while reading file");

    return input.split("\n").collect();
}
