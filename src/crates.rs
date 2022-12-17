use crate::utils::lines;
use regex::Regex;
use std::collections::VecDeque;
use std::vec::IntoIter;
use std::vec::Vec;
use lazy_static::lazy_static;

lazy_static! {
    static ref MOVE_ORDER_LINE_REGEX: Regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    static ref CRATES_LINE_REGEX: Regex = Regex::new(r"^(?:(?:(?: {3})|\[(?:[A-Z])\]) ?)+$").unwrap();
}

struct Crates {
    columns: Vec<VecDeque<char>>,
}

struct CrateMoveOrder {
    quantity: usize,
    from: usize,
    to: usize,
}

type CratesLine = IntoIter<Option<char>>;

enum CratesElt {
    CratesLine(CratesLine),
    CratesMoveOrder(CrateMoveOrder),
}

impl Crates {
    fn parse_line(line: &str) -> Option<CratesElt> {
        match line {
            crate_line if CRATES_LINE_REGEX.is_match(line) => {
                Some(CratesElt::CratesLine(Crates::get_crates(crate_line)))
            }
            crate_line if MOVE_ORDER_LINE_REGEX.is_match(line) => Some(CratesElt::CratesMoveOrder(
                Crates::create_move_order(crate_line),
            )),
            _ => None,
        }
    }

    fn get_crates(line: &str) -> CratesLine {
        let mut crate_chars: Vec<Option<char>> = Vec::new();
        let mut line_chars = line.chars().peekable();
        line_chars.next();
        while line_chars.peek().is_some() {
            match line_chars.next() {
                Some(line_char) if line_char != ' ' => crate_chars.push(Some(line_char)),
                Some(line_char) if line_char == ' ' => crate_chars.push(None),
                _ => continue,
            }
            line_chars.next();
            line_chars.next();
            line_chars.next();
        }
        crate_chars.into_iter()
    }

    fn create_move_order(line: &str) -> CrateMoveOrder {
        if !MOVE_ORDER_LINE_REGEX.is_match(line) {
            panic!("could not parse into a move order {}", line)
        }
        let mut order_line_matches = MOVE_ORDER_LINE_REGEX.captures_iter(line);
        match order_line_matches.next() {
            Some(captures) => CrateMoveOrder {
                quantity: usize::from_str_radix(&captures[1], 10).unwrap(),
                from: usize::from_str_radix(&captures[2], 10).unwrap(),
                to: usize::from_str_radix(&captures[3], 10).unwrap(),
            },
            _ => panic!("could not parse into a move order {}", line),
        }
    }

    fn move_crate(&mut self, move_order: CrateMoveOrder) {
        for  _ in 0..move_order.quantity {
            let moved_crate = self.columns[move_order.from - 1].pop_front().unwrap();
            self.columns[move_order.to - 1].push_front(moved_crate);
        }
    }
}

pub fn arrange(crates_filename: &str) -> String {
    let mut crates = Crates { columns: Vec::new()};
    if let Ok(lines) = lines(crates_filename) {
        for maybe_line in lines {
            if let Ok(line) = maybe_line {
                match Crates::parse_line(&line) {
                    Some(CratesElt::CratesLine(crates_line)) => {
                        if crates.columns.is_empty() {
                            crates.columns.extend((0..crates_line.len()).map(|_| VecDeque::new()));
                        }
                        for (idx, maybe_crate) in crates_line.enumerate() {
                            match maybe_crate {
                                Some(crate_elt) => crates.columns[idx].push_back(crate_elt),
                                _ => continue,
                            }
                        }
                    },
                    Some(CratesElt::CratesMoveOrder(move_order)) => {
                        crates.move_crate(move_order);
                    },
                    _ => continue,
                }
            }
        }
    };
    let mut result: String = "".to_owned();
    for opt_crate_char in crates.columns.iter().map(|deq| deq.front()) {
        match opt_crate_char {
            Some(crate_char) => result.push(*crate_char),
            _ => result.push_str(" "),
        };
    }
    result
}
