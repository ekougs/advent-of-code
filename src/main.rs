mod calories;
mod rps;
mod utils;
mod rucksack;

fn main() {
    // Day 1
    println!("Calories {}", calories::count_calories("calories_input.txt", 1));
    println!("Calories {}", calories::count_calories("calories_input.txt", 3));

    // Day 2
    println!("Rock paper scissor {}", rps::score_as_provided("rock_paper_scissor_input.txt"));
    println!("Rock paper scissor {}", rps::score_as_ordered("rock_paper_scissor_input.txt"));

    // Day 3
    println!("Rucksacks' priorities {}", rucksack::prioritize("rucksacks_input.txt"))
}
