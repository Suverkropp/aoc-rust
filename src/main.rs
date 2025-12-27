use aoc_rust::input;
use aoc_rust::year2025::day9;

fn main() {
    let input = input::get_input(2025, 9);
    let red_tiles = day9::handle_input(&input);
    let res = day9::part1(&red_tiles);
    println!("The result is {res}.")
}
