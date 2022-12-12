use crate::utils::apply_on_lines;
use std::fmt;
use std::cmp::Eq;
use std::cmp::PartialEq;
use std::hash::Hash;

#[derive(Eq, PartialEq, Hash)]
struct RPSElement {
    value: u32,
    symbols: [char; 2],
}

const ROCK: RPSElement = RPSElement {
    value: 1,
    symbols: ['A', 'X'],
};

const PAPER: RPSElement = RPSElement {
    value: 2,
    symbols: ['B', 'Y'],
};

const SCISSOR: RPSElement = RPSElement {
    value: 3,
    symbols: ['C', 'Z'],
};

const GAME: [RPSElement; 3] = [ROCK, PAPER, SCISSOR];

impl RPSElement {
    fn from(repr: char) -> RPSElement {
        for rps_elt in GAME {
            if rps_elt.symbols.contains(&repr) {
                return rps_elt;
            }
        }
        panic!("could not find RPSElement from {}", repr)
    }

    fn from_order(order: char, other: &RPSElement) -> &RPSElement {
        match order {
            // Lose
            'X' => &GAME[RPSElement::winning_against_idx(other)],
            // Draw
            'Y' => other,
            // Win
            'Z' => &GAME[RPSElement::losing_against_idx(other)],
            _ => panic!("Unknown order"),
        }
    }

    fn score(&self, other: &RPSElement) -> u32 {
        if self == other {
            return 3 + self.value
        }
        let score_against = if other == &GAME[RPSElement::winning_against_idx(self)] {
            6
        } else {
            0
        };
        score_against + self.value
    }

    fn game_idx(elt: &RPSElement) -> usize {
        match *elt {
            ROCK => 0,
            PAPER => 1,
            SCISSOR => 2,
            _ => panic!("Could not match rps element")
        }
    }

    fn winning_against_idx(elt: &RPSElement) -> usize {
        let game_idx = RPSElement::game_idx(elt);
        if game_idx == 0 {
            2
        } else {
            game_idx - 1
        }
    }

    fn losing_against_idx(elt: &RPSElement) -> usize {
        let game_idx = RPSElement::game_idx(elt);
        if game_idx == 2 {
            0
        } else {
            game_idx + 1
        }
    }
}

impl fmt::Display for RPSElement {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", self.value)
    }
}

pub fn score_as_provided(rps_filename: &str) -> u32 {
    let mut score: u32 = 0;
    apply_on_lines(rps_filename, |line: &str| -> () {
        let mut rps_str_elements = line.split(" ");
        let rps_against = map_rps_str(rps_str_elements.next(), RPSElement::from);
        let rps_for = map_rps_str(rps_str_elements.next(), RPSElement::from);
        let current_score = rps_for.score(&rps_against);
        score += current_score;
    });
    score
}

pub fn score_as_ordered(rps_filename: &str) -> u32 {
    let mut score: u32 = 0;
    apply_on_lines(rps_filename, |line: &str| -> () {
        let mut rps_str_elements = line.split(" ");
        let rps_against = map_rps_str(rps_str_elements.next(), RPSElement::from);
        let rps_for = map_rps_str(rps_str_elements.next(), |c| {
            RPSElement::from_order(c, &rps_against)
        });
        let current_score = rps_for.score(&rps_against);
        score += current_score;
    });
    score
}

fn map_rps_str<CM, T>(rps_str: Option<&str>, mut char_mapper: CM) -> T where CM: FnMut(char) -> T  {
    let rps_char: Option<char> = match rps_str.map(|r| r) {
        Some(s) => s.chars().next(),
        _ => panic!("couldn't find any string next on the line"),
    };
    match rps_char.map(|c| c) {
        Some(c) => char_mapper(c),
        _ => panic!("couldn't find any char on the str"),
    }
}
