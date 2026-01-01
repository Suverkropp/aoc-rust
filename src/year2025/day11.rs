use std::cmp::Ordering::{Equal, Greater, Less};
use itertools::Itertools;
use std::collections::HashMap;

type Code<'a> = &'a str;

pub fn handle_input(input: &'_ str) -> HashMap<Code<'_>, Vec<Code<'_>>> {
    let mut map = HashMap::new();
    let lines = input.lines();
    for line in lines {
        let (code, outputs) = parse_outputs(line);
        map.insert(code, outputs);
    }
    map
}

fn parse_outputs(line: &'_ str) -> (Code<'_>, Vec<Code<'_>>) {
    let (code, outputs) = line.split_once(":").unwrap();
    let outputs = outputs.split_whitespace().collect();
    (code, outputs)
}

pub fn part1(devices: &HashMap<Code, Vec<Code>>) -> u32 {
    find_paths(devices, "you", "out")
}

pub fn part2(devices: &HashMap<Code, Vec<Code>>) -> u64 {
    find_paths_via(devices, "svr", "out")
}

fn find_paths<'a>(devices: &HashMap<&'a str, Vec<&'a str>>, start: Code, end: Code) -> u32 {
    let mut paths_to_out: HashMap<Code, u32> = HashMap::new();
    paths_to_out.insert(end, 1);
    let reverse = reverse_map(devices);
    let mut to_check = reverse
        .get(end)
        .expect("Some device should output to \"out\"")
        .clone();
    while let Some(code) = to_check.pop() {
        let val: Option<u32> = devices
            .get(code)
            .expect("Each code should be in device list")
            .iter()
            .map(|output| paths_to_out.get(output))
            .sum();
        if let Some(val) = val {
            if code == start {
                println!("{val} paths found from {start} to {end}");
                return val;
            }
            paths_to_out.insert(code, val);
            to_check.append(&mut reverse.get(code).unwrap_or(&Vec::new()).clone())
        }
    }
    println!("No paths found from {start} to {end}");
    0
}

type CountStatus = (u64, bool, bool);

fn find_paths_via<'a>(devices: &HashMap<&'a str, Vec<&'a str>>, start: Code, end: Code) -> u64 {
    let mut paths_to_out: HashMap<Code, (u64, bool, bool)> = HashMap::new();
    paths_to_out.insert(end, (1, false, false));
    let reverse = reverse_map(devices);
    let mut to_check = reverse
        .get(end)
        .expect("Some device should output to \"out\"")
        .clone();
    while let Some(code) = to_check.pop() {
        let val: Option<(u64, bool, bool)> = devices
            .get(code)
            .expect("Each code should be in device list")
            .iter()
            .map(|output| paths_to_out.get(output))
            .fold_options((0, false, false), combine_counts);
        if let Some(res) = val {
            let (val, mut dac, mut fft) = res;
            if code == start {
                return if dac && fft {
                    val
                } else {
                    println!("No paths via dac and fft found");
                    0
                };
            } else if code == "dac" {
                dac = true;
            } else if code == "fft" {
                fft = true;
            }
            paths_to_out.insert(code, (val, dac, fft));
            to_check.append(&mut reverse.get(code).unwrap_or(&Vec::new()).clone())
        }
    }
    println!("No paths found from {start} to {end}");
    0
}

fn combine_counts((c1, dac1, fft1): CountStatus, (c2, dac2, fft2): &CountStatus) -> CountStatus {
    match (dac1, fft1).cmp( &(*dac2, *fft2)) {
        Less => (*c2, *dac2, *fft2),
        Equal => (c1+c2, dac1, fft1),
        Greater => (c1, dac1, fft1)
    }
}

fn reverse_map<'a>(map: &HashMap<&'a str, Vec<&'a str>>) -> HashMap<Code<'a>, Vec<Code<'a>>> {
    let mut reverse = HashMap::new();
    for (code, outputs) in map.iter() {
        for output in outputs {
            add_to_key(&mut reverse, *output, code)
        }
    }
    reverse
}

fn add_to_key<'a>(map: &mut HashMap<Code<'a>, Vec<Code<'a>>>, key: &'a str, value: &'a str) {
    if let Some(val) = map.get_mut(&key) {
        val.push(value);
    } else {
        map.insert(key, vec![value]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::get_test_input;

    #[test]
    pub fn part1() {
        let input = get_test_input(2025, 11);
        let devices = handle_input(&input);
        let res = super::part1(&devices);
        assert_eq!(res, 5);
    }

    #[test]
    pub fn part2() {
        let input = get_test_input(2025, 11_2);
        let devices = handle_input(&input);
        let res = super::part2(&devices);
        assert_eq!(res, 2);
    }
}
