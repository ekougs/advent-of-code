use crate::utils::apply_on_lines;
use std::collections::HashSet;

const LOWER_A_ASCII_VALUE: u32 = 'a' as u32;
const UPPER_A_ASCII_VALUE: u32 = 'A' as u32;


pub fn prioritize(rucksacks_filename: &str) -> u32 {
    let mut priorities_sum = 0;
    apply_on_lines(rucksacks_filename, |line| {
        let mut first_compartment: HashSet<char> = HashSet::new();
        for (idx, rucksack_item) in line.chars().enumerate() {
            if idx < line.len() / 2 {
                first_compartment.insert(rucksack_item);
            } else {
                if first_compartment.contains(&rucksack_item) {
                    priorities_sum += if rucksack_item.is_uppercase() {
                        rucksack_item as u32 - UPPER_A_ASCII_VALUE + 27
                    } else {
                        rucksack_item as u32 - LOWER_A_ASCII_VALUE + 1
                    };
                    break;
                }
            }
        }
    });
    priorities_sum
}