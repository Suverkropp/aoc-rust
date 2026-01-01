use aoc_rust::input;
use aoc_rust::year2025::day12;

fn main() {
    let input = input::get_input(2025, 12);
    let (presents, regions) = day12::handle_input(&input);
    let res = day12::part1(&presents, &regions);
    println!("The result is {res}.")
}
