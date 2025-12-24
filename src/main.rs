use std::fs;

fn main() {
    let contents = fs::read_to_string("input/2025/day1.txt").expect("No input file");

    let mut zeros = 0;
    let mut dial = 50;
    for line in contents.lines() {
        let first_letter = line.chars().nth(0);
        let sign = match first_letter {
            Some('L') => -1,
            Some('R') => 1,
            _ => panic!("Line doesn't start with L or R"),
        };
        let distance = line[1..].parse::<i32>().unwrap();
        dial = dial + sign * distance;
        dial = dial % 100;
        if dial == 0 {
            zeros += 1
        }
    }
    println!("{zeros}");
}
