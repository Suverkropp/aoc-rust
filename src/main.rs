use aoc_rust::input;
use aoc_rust::year2025::day8;

fn main() {
    let input = input::get_input(2025, 8);
    let junction_boxes = day8::handle_input(&input);
    let res = day8::part2(junction_boxes);
    println!("The result is {res}.")
}
