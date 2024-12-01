use std::{fs::OpenOptions, io::Read};

pub fn lines_from_file(filename: &str) -> Vec<String> {
    let path = String::from("input/") + filename;
    let mut file = OpenOptions::new().read(true).open(&path).unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();
    buffer.lines().map(|x| x.to_owned()).collect()
}
