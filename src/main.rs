mod calories;
mod rps;
pub mod utils;

fn main() {
    println!("Calories {}", calories::count_calories("calories_input.txt", 1));
    println!("Calories {}", calories::count_calories("calories_input.txt", 3));

    println!("Rock paper scissor {}", rps::score_as_provided("rock_paper_scissor_input.txt"));
    println!("Rock paper scissor {}", rps::score_as_ordered("rock_paper_scissor_input.txt"));
}
