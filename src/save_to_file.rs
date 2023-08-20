use std::fs::OpenOptions;
use std::io::prelude::*;

pub fn write<'a>(input: &'a String, path: &'a String, file_name: String) {
    let file_path = format!("{}{}", path.to_string(), file_name.to_string());
    // let file_path = PathBuf::from(file_path);

    let mut data_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_path)
        .expect("Could not open file");

    data_file
        .write(input.as_bytes())
        .expect("Could not write to project file");
}
