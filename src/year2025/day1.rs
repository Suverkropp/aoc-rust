use std::str::Lines;

pub fn handle_input(input: &String) -> Lines {
    input.lines()
}

fn parse_line(line: &str) -> i32 {
    let first_letter = line.chars().nth(0);
    let sign = match first_letter {
        Some('L') => -1,
        Some('R') => 1,
        _ => panic!("Line doesn't start with L or R"),
    };
    let distance = line[1..].parse::<i32>().unwrap();
    sign * distance
}

pub fn part1(lines: Lines) -> i32 {
    let mut zeros = 0;
    let mut dial = 50;
    for line in lines {
        let num = parse_line(line);
        dial = (dial + num) % 100;
        if dial == 0 {
            zeros += 1
        }
    }
    zeros
}

pub fn part2(lines: Lines) -> i32 {
    let mut zeros = 0;
    let mut dial = 50;
    for line in lines {
        let num = parse_line(line);
        let old = dial;
        dial = dial + num;
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