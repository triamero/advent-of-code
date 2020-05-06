use std::fs::File;
use std::io::Read;
use std::string::ToString;

pub fn read_lines(filename: &str) -> Vec<String> {
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => panic!(format!("File '{}' not found", filename))
    };

    let mut file_content: String = String::new();

    file.read_to_string(&mut file_content)
        .ok()
        .expect("Unable to read file");

    let lines: Vec<String> = file_content.split("\n")
        .map(|s: &str| s.trim().to_string())
        .collect();

    return lines;
}
