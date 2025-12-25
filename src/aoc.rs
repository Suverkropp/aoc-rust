use std::fs;

pub(crate) fn get_input(year: i32, day: i32) -> String {
    let path = format!("input/{year}/day{day}.txt");
    fs::read_to_string(path).expect("No input file")
}
