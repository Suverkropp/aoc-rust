pub fn handle_input(input: &str) -> impl Iterator<Item = (i32, i32)> {
    input.lines().map(parse_line)
}

fn parse_line(line: &str) -> (i32, i32) {
    let first_letter = line.chars().nth(0);
    let sign = match first_letter {
        Some('L') => -1,
        Some('R') => 1,
        _ => panic!("Line doesn't start with L or R"),
    };
    let distance = line[1..].parse::<i32>().unwrap();
    (sign, distance)
}

pub fn part1(rotations: impl Iterator<Item = (i32, i32)>) -> i32 {
    let mut zeros = 0;
    let mut dial = 50;
    for (dir, dist) in rotations {
        dial = (dial + dir * dist) % 100;
        if dial == 0 {
            zeros += 1
        }
    }
    zeros
}

pub fn part2(rotations: impl Iterator<Item = (i32, i32)>) -> i32 {
    let mut zeros = 0;
    let mut dial = 50;
    for (dir, dist) in rotations {
        let old = dial;
        dial = dial + dir * dist;
        if dial == 0 {
            zeros += 1
        }
        if old.signum() == -dial.signum() {
            zeros += 1;
        }
        zeros += (dial / 100).abs();
        dial = dial % 100;
    }
    zeros
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::get_test_input;

    #[test]
    pub fn part1() {
        let input = get_test_input(2025, 1);
        let input = handle_input(&input);
        let res = super::part1(input);
        assert_eq!(res, 3);
    }

    #[test]
    pub fn part2() {
        let input = get_test_input(2025, 1);
        let input = handle_input(&input);
        let res = super::part2(input);
        assert_eq!(res, 6);
    }
}
