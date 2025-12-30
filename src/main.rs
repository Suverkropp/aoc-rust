use aoc_rust::input;
use aoc_rust::year2025::day10;

fn main() {
    let input = input::get_input(2025, 10);
    let machines = day10::handle_input(&input);
    let res = day10::part2(&machines);
    println!("The result is {res}.")
}
