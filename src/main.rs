mod year2025;
mod aoc;

use year2025::day1;

fn main() {
    let input = aoc::get_input(2025, 1);
    let input = day1::handle_input(&input);
    let res = day1::part2(input);
    println!("The result is {res}.")
}

