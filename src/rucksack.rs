use crate::utils::apply_on_lines;
use std::collections::HashSet;
use std::collections::HashMap;

const LOWER_A_ASCII_VALUE: u32 = 'a' as u32;
const UPPER_A_ASCII_VALUE: u32 = 'A' as u32;


pub fn prioritize_for_diff_in_rumsack(rucksacks_filename: &str) -> u32 {
    let mut priorities_sum = 0;
    apply_on_lines(rucksacks_filename, |line| {
        let mut first_compartment: HashSet<char> = HashSet::new();
        for (idx, rucksack_item) in line.chars().enumerate() {
            if idx < line.len() / 2 {
                first_compartment.insert(rucksack_item);
            } else {
                if first_compartment.contains(&rucksack_item) {
                    priorities_sum += priority(rucksack_item);
                    break;
                }
            }
        }
    });
    priorities_sum
}

pub fn prioritize_for_group_common(rucksacks_filename: &str) -> u32 {
    let group_count = 3;
    let mut priorities_sum = 0;
    let mut nb_lines = 0;
    let mut lines_group_hashmap: HashMap<char, u8> = init_letter_counts(group_count);
    apply_on_lines(rucksacks_filename, |line| {
        nb_lines += 1;
        let line_char_set: HashSet<char> = HashSet::from_iter(line.chars());
        for rucksack_item in line_char_set {
            lines_group_hashmap.insert(rucksack_item, lines_group_hashmap[&rucksack_item] - 1);
        }
        if nb_lines == group_count {
            let common_rucksack_item_entry = lines_group_hashmap.iter().filter(|entry| *(entry.1) == 0).next();
            match common_rucksack_item_entry {
                Some((common_rucksack_item, _)) =>  priorities_sum += priority(*common_rucksack_item),
                _ => panic!("should at least have foudn a common rumsack item"),
            };
            // Reset for the next group
            nb_lines = 0;
            lines_group_hashmap = init_letter_counts(group_count);
        }
    });
    priorities_sum
}

fn priority(rucksack_item: char) -> u32 {
    if rucksack_item.is_uppercase() {
        rucksack_item as u32 - UPPER_A_ASCII_VALUE + 27
    } else {
        rucksack_item as u32 - LOWER_A_ASCII_VALUE + 1
    }
}

fn init_letter_counts(group_count: u8) -> HashMap<char, u8> {
    (b'A'..=b'z')
    .map(|c| c as char)
    .filter(|c| c.is_alphabetic())
    .map(|c| (c, group_count))
    .collect::<HashMap<char, u8>>()
}
