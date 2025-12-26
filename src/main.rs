mod aoc;
mod year2025;

use year2025::day4;

fn main() {
    let input = aoc::get_input(2025, 4);
    let input = day4::handle_input(&input);
    let res = day4::part1(&input);
    println!("The result is {res}.")
}
