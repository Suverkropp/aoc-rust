pub enum Op {
    Add,
    Times,
}

pub fn handle_input(input: &str) -> Vec<&str> {
    input.lines().collect::<Vec<&str>>()
}

fn read_nums(line: &&str) -> Vec<u64> {
    let words = line.split_whitespace();
    words.map(|x| x.parse::<u64>().unwrap()).collect()
}

fn read_ops(line: &str) -> Vec<Op> {
    line.split_whitespace().map(read_op).collect()
}

fn read_op(input: &str) -> Op {
    match input {
        "+" => Op::Add,
        "*" => Op::Times,
        _ => panic!("Invalid operator: {}", input),
    }
}

pub fn part1(input: Vec<&str>) -> u64 {
    let nums: Vec<_> = input[0..input.len() - 1].iter().map(read_nums).collect();
    let ops: Vec<_> = read_ops(input[input.len() - 1]);
    let nums = transpose(nums);
    compute_grand_total(&nums, &ops)
}

fn compute_grand_total(problems: &[Vec<u64>], ops: &[Op]) -> u64 {
    let mut grand_total = 0;
    for (problem, op) in problems.iter().zip(ops.iter()) {
        grand_total += match op {
            Op::Add => problem.iter().sum::<u64>(),
            Op::Times => problem.iter().product(),
        }
    }
    grand_total
}

fn transpose<T: Copy>(vec: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut new_vec = Vec::new();
    for i in 0..vec[0].len() {
        new_vec.push(vec.iter().map(|row| row[i]).collect());
    }
    new_vec
}

pub fn part2(lines: Vec<&str>) -> u64 {
    let ops = read_ops(lines[lines.len() - 1]);
    let bytes = lines[0..lines.len() - 1]
        .iter()
        .map(|s| s.bytes().collect())
        .collect();
    let columns: Vec<_> = transpose(bytes)
        .iter()
        .map(|col| str::from_utf8(col).unwrap())
        .map(|col| col.trim().parse::<u64>())
        .collect();

    let mut problems = Vec::new();
    let mut current_problem = Vec::new();
    for col in columns {
        match col {
            Ok(num) => current_problem.push(num),
            Err(_) => {
                problems.push(current_problem);
                current_problem = Vec::new();
            }
        }
    }
    problems.push(current_problem);
    
    compute_grand_total(&problems, &ops)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::get_test_input;

    #[test]
    pub fn part1() {
        let input = get_test_input(2025, 6);
        let lines = handle_input(&input);
        let res = super::part1(lines);
        assert_eq!(res, 4277556);
    }

    #[test]
    pub fn part2() {
        let input = get_test_input(2025, 6);
        let lines = handle_input(&input);
        let res = super::part2(lines);
        assert_eq!(res, 3263827);
    }
}
