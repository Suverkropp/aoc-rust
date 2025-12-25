mod aoc;
mod year2025;

use year2025::day2;

fn main() {
    let input = aoc::get_input(2025, 2);
    let input = day2::handle_input(&input);
    let res = day2::part2(input);
    println!("The result is {res}.")
}
