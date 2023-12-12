use std::fs::File;
use std::io::Read;

pub fn read_from_file(filename: &str) -> String {
    let mut file = File::open(filename).unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();

    buffer
}
