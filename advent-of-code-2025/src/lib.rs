use std::fs::File;
use std::io::Read;

pub fn read_from_file(filename: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(filename).unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    Ok(buffer)
}

pub fn is_same_vec(vec1: &Vec<i32>, vec2: &Vec<i32>) -> bool {
    let matching = vec1
        .iter()
        .zip(vec2.iter())
        .filter(|&(a, b)| a == b)
        .count();
    matching == vec1.len() && matching == vec2.len()
}
