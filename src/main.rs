

mod calories;

fn main() {
    println!("Calories {}", calories::count_calories("calories_input.txt", 1));
    println!("Calories {}", calories::count_calories("calories_input.txt", 3));
}
