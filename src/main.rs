use aoc_rust::year2025::day5;
use aoc_rust::input;

fn main() {
    let input = input::get_input(2025, 5);
    let (ranges, _ids) = day5::handle_input(&input);
    let res = day5::part2(&ranges);
    println!("The result is {res}.")
}
