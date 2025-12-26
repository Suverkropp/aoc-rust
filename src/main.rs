use aoc_rust::year2025::day6;
use aoc_rust::input;

fn main() {
    let input = input::get_input(2025, 6);
    let (nums, ops) = day6::handle_input(&input);
    let res = day6::part1(&nums, &ops);
    println!("The result is {res}.")
}
