use std::fs;

pub fn get_input(year: i32, day: i32) -> String {
    let path = format!("input/{year}/day{day}.txt");
    fs::read_to_string(path).expect("No input file")
}

pub fn get_test_input(year: i32, day: i32) -> String {
    let path = format!("input/{year}/test{day}.txt");
    fs::read_to_string(path).expect("No input file")
}