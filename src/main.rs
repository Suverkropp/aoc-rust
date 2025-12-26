mod aoc;
mod year2025;

use year2025::day5;

fn main() {
    let input = aoc::get_input(2025, 5);
    let (ranges, _ids) = day5::handle_input(&input);
    let res = day5::part2(&ranges);
    println!("The result is {res}.")
}
