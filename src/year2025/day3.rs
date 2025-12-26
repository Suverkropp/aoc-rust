pub fn handle_input(input: &str) -> impl Iterator<Item = Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|i| i.to_digit(10).unwrap()).collect())
}

pub fn part1(input: impl Iterator<Item = Vec<u32>>) -> u32 {
    input.map(bank_joltage).sum()
}

fn bank_joltage(bank: Vec<u32>) -> u32 {
    let all_but_last = &bank[0..bank.len() - 1];
    let first_battery = all_but_last.iter().max().unwrap();
    let index = bank.iter().position(|&i| i == *first_battery).unwrap();
    let after_first = &bank[index + 1..];
    let second_battery = after_first.iter().max().unwrap();
    let max_joltage = first_battery * 10 + second_battery;
    max_joltage
}

pub fn part2(input: impl Iterator<Item = Vec<u32>>) -> u64 {
    input.map(bank_joltage_with_override).sum()
}

fn bank_joltage_with_override(bank: Vec<u32>) -> u64 {
    let mut num: u64 = 0;
    let mut start_index = 0;
    for i in 0..12 {
        let to_search = &bank[start_index..bank.len() - 11 + i];
        let battery = to_search.iter().max().unwrap();
        start_index += to_search.iter().position(|&i| i == *battery).unwrap() + 1;
        num = 10 * num + (*battery as u64)
    }
    num
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::get_test_input;

    #[test]
    pub fn part1() {
        let input = get_test_input(2025, 3);
        let input = handle_input(&input);
        let res = super::part1(input);
        assert_eq!(res, 357);
    }

    #[test]
    pub fn part2() {
        let input = get_test_input(2025, 3);
        let input = handle_input(&input);
        let res = super::part2(input);
        assert_eq!(res, 3121910778619);
    }
}
