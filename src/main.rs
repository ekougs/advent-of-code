mod calories;
mod crates;
mod cube_conundrum;
mod dirs;
mod packet;
mod pairs;
mod rps;
mod rucksack;
mod trebuchet;
mod trees;
mod utils;

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
// Dir size < 100_000 3579501
// Nb of visible trees 1684
// Trebuchet digits 56506
// Trebuchet digits and letters 56017
// Cube conundrum 1931
// Cube conundrum (Power sum) 83105

fn main() {
    // 2022
    // Day 1
    println!(
        "Calories {}",
        calories::count_calories("calories_input.txt", 1)
    );
    println!(
        "Calories {}",
        calories::count_calories("calories_input.txt", 3)
    );

    // Day 2
    println!(
        "Rock paper scissor {}",
        rps::score_as_provided("rock_paper_scissor_input.txt")
    );
    println!(
        "Rock paper scissor {}",
        rps::score_as_ordered("rock_paper_scissor_input.txt")
    );

    // Day 3
    println!(
        "Rucksacks' priorities {}",
        rucksack::prioritize_for_diff_in_rumsack("rucksacks_input.txt")
    );
    println!(
        "Rucksacks' priorities {}",
        rucksack::prioritize_for_group_common("rucksacks_input.txt")
    );

    // Day 4
    println!(
        "Covering pairs {}",
        pairs::covering("covering_pairs_input.txt")
    );
    println!(
        "Covering pairs {}",
        pairs::overlaping("covering_pairs_input.txt")
    );

    // Day 5
    println!(
        "Arrange crates {}",
        crates::arrange_one_by_one("crates_input.txt")
    );
    println!(
        "Arrange crates {}",
        crates::arrange_mult("crates_input.txt")
    );

    // Day 6
    println!(
        "Marker starts at {}",
        packet::start_idx("start_of_packet_marker_input.txt", 4)
    );
    println!(
        "Message starts at {}",
        packet::start_idx("start_of_packet_marker_input.txt", 14)
    );

    // Day 7
    println!(
        "Dir size < 100_000 {}",
        dirs::dirs_size("candidate_directories_input.txt", 100_000)
    );
    println!(
        "Dir size < 100_000 {}",
        dirs::min_dir_size_to_free("candidate_directories_input.txt", 70_000_000, 30_000_000)
    );

    // Day 8
    println!(
        "Nb of visible trees {}",
        trees::nb_visible_trees("tree_house_input.txt")
    );

    // 2023
    // Day 1
    println!(
        "Trebuchet {}",
        trebuchet::calibrate_using_digits("trebuchet_input.txt")
    );
    println!(
        "Trebuchet with letters {}",
        trebuchet::calibrate_using_letters_and_digits("trebuchet_input.txt")
    );

    // Day 2
    println!(
        "Cube conundrum {}",
        cube_conundrum::possible_games("cube_conundrum_input.txt")
    );
    println!(
        "Cube conundrum (Power sum) {}",
        cube_conundrum::power_sum("cube_conundrum_input.txt")
    );
}
