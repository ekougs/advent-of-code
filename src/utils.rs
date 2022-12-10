use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn apply_on_lines<F>(filename: &str, f: F) where F: FnMut(&str) -> () {
    // Create a path to the desired file
    let rps_input_path = Path::new(filename);
    let display = rps_input_path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&rps_input_path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut rps_file_content = String::new();
    match file.read_to_string(&mut rps_file_content) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => {}
    };

    rps_file_content.lines().for_each(f)
}
