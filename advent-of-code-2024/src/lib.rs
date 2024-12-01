use std::fs::File;
use std::io::Read;

pub fn read_from_file(filename: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(filename).unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    Ok(buffer)
}
