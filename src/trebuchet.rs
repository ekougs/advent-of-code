use crate::utils::lines;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref DIGITS_OR_LETTERS_REGEX: Regex =
        Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|\d)").unwrap();
    static ref LETTERS_TO_DIGIT: HashMap<&'static str, u32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9)
    ],);
}

pub fn calibrate_using_letters_and_digits(trebuchet_filename: &str) -> u32 {
    match lines(trebuchet_filename) {
        Err(why) => {
            panic!("Couldn't open {}: {}", trebuchet_filename, why)
        }
        Ok(mut lines) => {
            let mut total_calibration = 0;
            while let Some(Ok(line)) = lines.next() {
                total_calibration += to_calibration_letters_and_digits(&line);
            }
            total_calibration
        }
    }
}

pub fn calibrate_using_digits(trebuchet_filename: &str) -> u32 {
    match lines(trebuchet_filename) {
        Err(why) => {
            panic!("Couldn't open {}: {}", trebuchet_filename, why)
        }
        Ok(mut lines) => {
            let mut total_calibration = 0;
            while let Some(Ok(line)) = lines.next() {
                total_calibration += to_calibration_digits(line);
            }
            total_calibration
        }
    }
}

fn to_calibration_digits(line: String) -> u32 {
    let mut first_digit: Option<u32> = None;
    let mut last_digit: Option<u32> = None;
    let mut line_chars = line.chars().peekable();
    while let Some(char) = line_chars.next() {
        match char {
            // The first calibration digit
            char if char.is_ascii_digit() && first_digit.is_none() => {
                first_digit = Some(char.to_digit(10).unwrap())
            }
            // The second calibration digit
            char if char.is_ascii_digit() && first_digit.is_some() => {
                last_digit = Some(char.to_digit(10).unwrap())
            }
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

fn to_calibration_letters_and_digits(line: &str) -> u32 {
    let mut first_digit: Option<u32> = None;
    let mut last_digit: Option<u32> = None;
    let mut locs = DIGITS_OR_LETTERS_REGEX.capture_locations();
    let mut start = 0;
    let mut opt_captures = DIGITS_OR_LETTERS_REGEX.captures_read_at(&mut locs, line, start);

    while let Some(captures) = opt_captures {
        let digit_or_letters = captures.as_str();
        let digit: u32 = match digit_or_letters {
            letters if LETTERS_TO_DIGIT.contains_key(&letters) => {
                *LETTERS_TO_DIGIT.get(&letters).unwrap()
            }
            _ => digit_or_letters
                .chars()
                .next()
                .unwrap()
                .to_digit(10)
                .unwrap(),
        };
        if first_digit.is_none() {
            first_digit = Some(digit);
        } else {
            last_digit = Some(digit);
        }
        start = locs.get(1).unwrap().0 + 1;
        opt_captures = DIGITS_OR_LETTERS_REGEX.captures_read_at(&mut locs, line, start);
    }
    if first_digit.is_none() {
        first_digit = Some(0);
    }
    if last_digit.is_none() {
        last_digit = first_digit;
    }

    // println!("Calibration {}", first_digit.unwrap() * 10 + last_digit.unwrap());
    first_digit.unwrap() * 10 + last_digit.unwrap()
}
