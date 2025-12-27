use aoc_rust::year2025::day7;
use aoc_rust::input;

fn main() {
    let input = input::get_input(2025, 7);
    let (start, splitters) = day7::handle_input(&input);
    let res = day7::part2(start, splitters);
    println!("The result is {res}.")
}
