use aoc_rust::input;
use aoc_rust::year2025::day11;

fn main() {
    let input = input::get_input(2025, 11);
    let devices = day11::handle_input(&input);
    let res = day11::part1(&devices);
    println!("The result is {res}.")
}
