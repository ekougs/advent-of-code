use min_max_heap::MinMaxHeap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use crate::utils::lines;

pub fn count_calories(calories_filename: &str, topn: usize) -> i64 {
    // Create a path to the desired file
    let calories_input_path = Path::new(calories_filename);
    let display = calories_input_path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&calories_input_path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut calories_file_content = String::new();
    match file.read_to_string(&mut calories_file_content) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => {}
    };

    let mut current_calories = 0;
    let mut topn_calories: MinMaxHeap<i64> = MinMaxHeap::with_capacity(topn);

    if let Ok(lines) = lines(calories_filename) {
        for maybe_line in lines {
            if let Ok(line) = maybe_line {
                match line.parse::<i64>().map(|i| i) {
                    Ok(n) => current_calories += n,
                    Err(..) => {
                        if topn_calories.len() < topn {
                            topn_calories.push(current_calories);
                        } else if topn_calories.peek_min().map_or(0, |calories| *calories)
                            < current_calories
                        {
                            topn_calories.pop_min();
                            topn_calories.push(current_calories);
                        }
                        current_calories = 0;
                    }
                }
            }
        }
    }
    topn_calories.iter().sum()
}
