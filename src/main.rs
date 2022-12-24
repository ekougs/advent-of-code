mod calories;
mod rps;
mod utils;
mod rucksack;
mod pairs;
mod crates;
mod packet;
mod dirs;

// Calories 72478
// Calories 210367
// Rock paper scissor 15572
// Rock paper scissor 16098
// Rucksacks' priorities 7581
// Rucksacks' priorities 2525
// Covering pairs 532
// Covering pairs 854
// Arrange crates CWMTGHBDW
// Arrange crates SSCGWJCRB
// Marker starts at 1034
// Message starts at 2472
// Dir size < 100_000 1477771

fn main() {
    // Day 1
    println!("Calories {}", calories::count_calories("calories_input.txt", 1));
    println!("Calories {}", calories::count_calories("calories_input.txt", 3));

    // Day 2
    println!("Rock paper scissor {}", rps::score_as_provided("rock_paper_scissor_input.txt"));
    println!("Rock paper scissor {}", rps::score_as_ordered("rock_paper_scissor_input.txt"));

    // Day 3
    println!("Rucksacks' priorities {}", rucksack::prioritize_for_diff_in_rumsack("rucksacks_input.txt"));
    println!("Rucksacks' priorities {}", rucksack::prioritize_for_group_common("rucksacks_input.txt"));

    // Day 4
    println!("Covering pairs {}", pairs::covering("covering_pairs_input.txt"));
    println!("Covering pairs {}", pairs::overlaping("covering_pairs_input.txt"));

    // Day 5
    println!("Arrange crates {}", crates::arrange_one_by_one("crates_input.txt"));
    println!("Arrange crates {}", crates::arrange_mult("crates_input.txt"));

    // Day 6
    println!("Marker starts at {}", packet::start_idx("start_of_packet_marker_input.txt", 4));
    println!("Message starts at {}", packet::start_idx("start_of_packet_marker_input.txt", 14));

    // Day 7
    println!("Dir size < 100_000 {}", dirs::dirs_size("candidate_directories_input.txt", 100_000))
}
