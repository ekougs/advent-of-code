use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn count_calories() -> i64 {

    // Create a path to the desired file
    let calories_input_path = Path::new("calories_input_1_2.txt");
    let display = calories_input_path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&calories_input_path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut calories_file_content = String::new();
    match file.read_to_string(&mut calories_file_content) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => {},
    };

    let mut max_calories = 0;
    let mut current_calories = 0;
    for line in calories_file_content.lines() {
        match line.parse::<i64>().map(|i| i) {
            Ok(n) => current_calories += n,
            Err(..) => {
                if max_calories < current_calories {
                    max_calories = current_calories;
                }
                current_calories = 0;
            }
        }
    }
    max_calories
}
