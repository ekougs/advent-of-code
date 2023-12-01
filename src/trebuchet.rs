use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use crate::utils::lines;

pub fn calibrate(trebuchet_filename: &str) -> u32 {
    let trebuchet_input_path = Path::new(trebuchet_filename);
    let display = trebuchet_input_path.display();
    
    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&trebuchet_input_path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    
    let mut calories_file_content = String::new();
    match file.read_to_string(&mut calories_file_content) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => {}
    };

    let mut total_calibration = 0;
    if let Ok(lines) = lines(trebuchet_filename) {
        for maybe_line in lines {
            if let Ok(line) = maybe_line {
                total_calibration += to_calibration(line)
            }
        }
    }
    total_calibration
}

fn to_calibration(line: String) -> u32 {
    let mut first_digit: Option<u32> = None;
    let mut last_digit: Option<u32> = None;
    let mut line_chars = line.chars().peekable();
    while line_chars.peek().is_some() {
        match line_chars.next() {
            // The first calibration digit
            Some(line_char) if line_char.is_ascii_digit() && first_digit.is_none() => first_digit = Some(line_char.to_digit(10).unwrap()),
            // The second calibration digit
            Some(line_char) if line_char.is_ascii_digit() && first_digit.is_some() => last_digit = Some(line_char.to_digit(10).unwrap()),
            _ => continue,
        }
    }
    if first_digit.is_none() {
        first_digit = Some(0);
    }
    if last_digit.is_none() {
        last_digit = first_digit;
    }
    first_digit.unwrap() * 10 + last_digit.unwrap()
}