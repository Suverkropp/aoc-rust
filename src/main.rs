mod aoc;
mod year2025;

use year2025::day3;

fn main() {
    let input = aoc::get_input(2025, 3);
    let input = day3::handle_input(&input);
    let res = day3::part2(input);
    println!("The result is {res}.")
}
