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

pub fn part2(devices: &HashMap<Code, Vec<Code>>) -> u32 {
    let dac_to_out = find_paths(devices, "dac", "out");
    let fft_to_out = find_paths(devices, "fft", "out");
    let dac_to_fft = find_paths(devices, "dac", "fft");
    let fft_to_dac = find_paths(devices, "fft", "dac");
    let srv_to_fft = find_paths(devices, "srv", "fft");
    let srv_to_dac = find_paths(devices, "srv", "dac");
    srv_to_fft * fft_to_dac * dac_to_out + srv_to_dac * dac_to_fft * fft_to_out
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
            to_check.append(&mut reverse.get(code).unwrap().clone())
        }
    }
    println!("No paths found from {start} to {end}");
    0
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
        let input = get_test_input(2025, 11);
        let devices = handle_input(&input);
        let res = super::part2(&devices);
        assert_eq!(res, 2);
    }
}
