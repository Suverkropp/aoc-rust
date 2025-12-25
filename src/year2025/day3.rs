pub fn handle_input(input: &str) -> impl Iterator<Item = Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|i| i.to_digit(10).unwrap()).collect())
}

pub fn part1(input: impl Iterator<Item = Vec<u32>>) -> u32 {
    input.map(bank_joltage).sum()
}

fn bank_joltage(bank: Vec<u32>) -> u32 {
    let all_but_last = &bank[0..bank.len()-1];
    let first_battery = all_but_last.iter().max().unwrap();
    let index = bank.iter().position(|&i| &i == first_battery).unwrap();
    let after_first = &bank[index+1..];
    let second_battery = after_first.iter().max().unwrap();
    let max_joltage = first_battery * 10 + second_battery;
    max_joltage
}
