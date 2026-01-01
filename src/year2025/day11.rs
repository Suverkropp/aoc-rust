use itertools::Itertools;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::HashMap;
use std::fmt::Debug;

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
    find_paths(devices, "you", "out", 1, 0, |a, b| a + b, |_, v| v)
}

fn find_paths<'a, T: Debug + Clone>(
    devices: &HashMap<Code, Vec<Code>>,
    start: Code,
    end: Code,
    initial: T,
    default: T,
    fold_func: impl Fn(T, &T) -> T,
    adapt: impl Fn(Code, T) -> T,
) -> T {
    let mut values: HashMap<Code, T> = HashMap::new();
    values.insert(end, initial);
    let reverse = reverse_map(devices);
    let mut to_check = reverse
        .get(end)
        .expect("Some device should output to \"out\"")
        .clone();
    while let Some(code) = to_check.pop() {
        let val: Option<T> = devices
            .get(code)
            .expect("Each code should be in device list")
            .iter()
            .map(|output| values.get(output))
            .fold_options(default.clone(), |a, b| fold_func(a, b));
        if let Some(val) = val {
            if code == start {
                println!("{val:?} paths found from {start} to {end}");
                return val;
            }
            let val = adapt(code, val);
            values.insert(code, val);
            to_check.append(&mut reverse.get(code).unwrap_or(&Vec::new()).clone())
        }
    }
    println!("No paths found from {start} to {end}");
    default
}

fn reverse_map<'a>(map: &HashMap<Code<'a>, Vec<Code<'a>>>) -> HashMap<Code<'a>, Vec<Code<'a>>> {
    let mut reverse = HashMap::new();
    for (code, outputs) in map.iter() {
        for output in outputs {
            add_to_key(&mut reverse, *output, code)
        }
    }
    reverse
}

fn add_to_key<'a>(map: &mut HashMap<Code<'a>, Vec<Code<'a>>>, key: Code<'a>, value: Code<'a>) {
    if let Some(val) = map.get_mut(&key) {
        val.push(value);
    } else {
        map.insert(key, vec![value]);
    }
}

type CountStatus = (u64, bool, bool);
pub fn part2(devices: &HashMap<Code, Vec<Code>>) -> u64 {
    find_paths(
        devices,
        "svr",
        "out",
        (1, false, false),
        (0, false, false),
        combine_counts,
        check_dac_or_fft,
    )
    .0
}

fn combine_counts((c1, dac1, fft1): CountStatus, (c2, dac2, fft2): &CountStatus) -> CountStatus {
    match (dac1, fft1).cmp(&(*dac2, *fft2)) {
        Less => (*c2, *dac2, *fft2),
        Equal => (c1 + c2, dac1, fft1),
        Greater => (c1, dac1, fft1),
    }
}

fn check_dac_or_fft(code: Code, (c, dac, fft): CountStatus) -> CountStatus {
    if code == "dac" {
        (c, true, fft)
    } else if code == "fft" {
        (c, dac, true)
    } else {
        (c, dac, fft)
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
