use crate::utils::lines;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref GAME_REGEX: Regex = Regex::new(r"Game (\d+): ").unwrap();
    static ref CUBE_REGEX: Regex = Regex::new(r"(\d+) (red|blue|green)").unwrap();
}

pub fn possible_games(cube_conundrum_filename: &str) -> u32 {
    match lines(cube_conundrum_filename) {
        Err(why) => {
            panic!("Couldn't open {}: {}", cube_conundrum_filename, why)
        }
        Ok(mut lines) => {
            let mut possible_games_total = 0;
            while let Some(Ok(line)) = lines.next() {
                possible_games_total += possible_game_nb(line);
            }
            possible_games_total
        }
    }
}

pub fn power_sum(cube_conundrum_filename: &str) -> u32 {
    match lines(cube_conundrum_filename) {
        Err(why) => {
            panic!("Couldn't open {}: {}", cube_conundrum_filename, why)
        }
        Ok(mut lines) => {
            let mut power_total = 0;
            while let Some(Ok(line)) = lines.next() {
                power_total += fewest_cube_power(line);
            }
            power_total
        }
    }
}

fn possible_game_nb(line: String) -> u32 {
    let cubes_by_color: HashMap<&str, u32> =
        HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    for (_, [nb_cubes_str, color]) in CUBE_REGEX.captures_iter(&line).map(|c| c.extract()) {
        let cubes_for_color = cubes_by_color.get(color).unwrap();
        let nb_cubes = nb_cubes_str.parse::<u32>().unwrap();
        if nb_cubes > *cubes_for_color {
            return 0;
        }
    }
    match GAME_REGEX.captures(&line) {
        Some(captures) => captures.get(1).unwrap().as_str().parse::<u32>().unwrap(),
        _ => panic!("Could not match with the game regex for line {}", line),
    }
}

fn fewest_cube_power(line: String) -> u32 {
    let mut max_cubes_by_color: HashMap<&str, u32> =
        HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
    for (_, [nb_cubes_str, color]) in CUBE_REGEX.captures_iter(&line).map(|c| c.extract()) {
        let max_cubes_for_color = max_cubes_by_color.get(color).unwrap();
        let nb_cubes = nb_cubes_str.parse::<u32>().unwrap();
        if nb_cubes > *max_cubes_for_color {
            max_cubes_by_color.insert(color, nb_cubes);
        }
    }
    max_cubes_by_color
        .values()
        .map(|a| *a)
        .reduce(|a, b| a * b)
        .unwrap()
}
