use crate::utils::apply_on_lines;

struct Pair {
    start: usize,
    end: usize,
}

impl Pair {
    fn from(opt_pair_str: Option<&str>) -> Pair {
        let pair_str = match opt_pair_str {
            Some(pair_str) => pair_str,
            _ => panic!("could not extract a string from input")
        };
        let mut pair_elts_str = pair_str.split("-");
        Pair {
            start: Pair::parse_pair_elt(pair_elts_str.next(), pair_str),
            end: Pair::parse_pair_elt(pair_elts_str.next(), pair_str),
        }
    }

    fn parse_pair_elt(opt_pair_elt_str: Option<&str>, pair_str: &str) -> usize {
        let pair_elt_str = match opt_pair_elt_str {
            Some(elt_str) => elt_str,
            _ => panic!("could not find a str in pair {}", pair_str),
        };
        match usize::from_str_radix(pair_elt_str, 10) {
            Ok(size) => size,
            _ => panic!("could not convert the first part of the pair {}", pair_str)
        }
    }

    fn contains(&self, other: &Pair) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps_with(&self, other: &Pair) -> bool {
        self.contains_extremity(other) || other.contains_extremity(self)
    }

    fn contains_extremity(&self, other: &Pair) -> bool {
        if self.start <= other.start && other.start <= self.end {
            return true
        }
        if self.start <= other.end && other.end <= self.end {
            return true
        }
        false
    }
}


pub fn covering(pairs_filename: &str) -> u32 {
    let mut covering_pairs = 0;
    apply_on_lines(pairs_filename, |line| {
        let mut pairs_str = line.split(",");
        let first_pair = Pair::from(pairs_str.next());
        let second_pair = Pair::from(pairs_str.next());
        if first_pair.contains(&second_pair) || second_pair.contains(&first_pair) {
            covering_pairs += 1;
        }
    });
    covering_pairs
}

pub fn overlaping(pairs_filename: &str) -> u32 {
    let mut overlaping_pairs = 0;
    apply_on_lines(pairs_filename, |line| {
        let mut pairs_str = line.split(",");
        let first_pair = Pair::from(pairs_str.next());
        let second_pair = Pair::from(pairs_str.next());
        if first_pair.overlaps_with(&second_pair) {
            overlaping_pairs += 1;
        }
    });
    overlaping_pairs
}