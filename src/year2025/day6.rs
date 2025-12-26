pub enum Op {
    Add,
    Times,
}

pub fn handle_input(input: &str) -> (Vec<Vec<u64>>, Vec<Op>) {
    let lines = input.lines().collect::<Vec<&str>>();
    let values = lines[0..lines.len() - 1]
        .iter()
        .map(read_nums)
        .collect();
    let ops = lines[lines.len() - 1]
        .split_whitespace()
        .map(read_op)
        .collect();
    (values, ops)
}

fn read_nums(line: &&str) -> Vec<u64> {
    let words = line.split_whitespace();
    words.map(|x| x.parse::<u64>().unwrap()).collect()
}

fn read_op(input: &str) -> Op {
    match input {
        "+" => Op::Add,
        "*" => Op::Times,
        _ => panic!("Invalid operator: {}", input),
    }
}

pub fn part1(nums: &[Vec<u64>], ops: &[Op]) -> u64 {
    let mut grand_total = 0;
    for (i, op) in ops.iter().enumerate() {
        let problem = nums.iter().map(|row| row[i]);
        grand_total += match op {
            Op::Add => { problem.sum::<u64>() }
            Op::Times => { problem.product() }
        }
    }
    grand_total
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::get_test_input;

    #[test]
    pub fn part1() {
        let input = get_test_input(2025, 6);
        let (nums, ops) = handle_input(&input);
        let res = super::part1(&nums, &ops);
        assert_eq!(res, 4277556);
    }
}
