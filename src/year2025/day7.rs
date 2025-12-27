pub fn handle_input(input: &str) -> (usize, Vec<Vec<bool>>) {
    let start = input.bytes().position(|byte| byte == b'S').unwrap();
    let splitters = input.lines().map(|line| {
        line.bytes().map(|b| is_splitter(b)).collect()
    }).collect();
    (start, splitters)
}

fn is_splitter(byte: u8) -> bool {
    match byte {
        b'^' => true,
        b'.' => false,
        b'S' => false,
        _ => panic!("Invalid byte: {}", byte),
    }
}

pub fn part1(start: usize, splitters: Vec<Vec<bool>>) -> usize {
    let mut tachyon_beams = vec![false; splitters[0].len()];
    tachyon_beams[start] = true;
    let mut splits = 0;
    for layer in splitters {
        splits += split_and_count(layer, &mut tachyon_beams);
    }
    splits
}

fn split_and_count(splitters: Vec<bool>, tachyon_beams: &mut Vec<bool>) -> usize {
    let mut new_beams = vec![false; splitters.len()];
    let mut splits = 0;
    for (i, beam) in tachyon_beams.iter().enumerate() {
        if !*beam {continue;}
        if splitters[i] {
            splits += 1;
            new_beams[i-1] = true;
            new_beams[i+1] = true;
        } else {
            new_beams[i] = true;
        }
    }
    *tachyon_beams = new_beams;
    splits
}

pub fn part2(start: usize, splitters: Vec<Vec<bool>>) -> u64 {
    let mut tachyon_beams = vec![0; splitters[0].len()];
    tachyon_beams[start] = 1;
    let mut timelines = 1;
    for layer in splitters {
        timelines += quantum_split_and_count(layer, &mut tachyon_beams);
    }
    timelines
}

fn quantum_split_and_count(splitters: Vec<bool>, tachyon_beams: &mut Vec<u64>) -> u64 {
    let mut new_beams = vec![0; splitters.len()];
    let mut splits = 0;
    for (i, beams) in tachyon_beams.iter().enumerate() {
        if splitters[i] {
            splits += beams;
            new_beams[i-1] += beams;
            new_beams[i+1] += beams;
        } else {
            new_beams[i] += beams;
        }
    }
    *tachyon_beams = new_beams;
    splits
}

#[cfg(test)]
mod tests {
    use crate::input::get_test_input;
    use super::*;

    #[test]
    pub fn part1() {
        let input = get_test_input(2025, 7);
        let (start, splitters) = handle_input(&input);
        let res = super::part1(start, splitters);
        assert_eq!(res, 21);
    }

    #[test]
    pub fn part2() {
        let input = get_test_input(2025, 7);
        let (start, splitters) = handle_input(&input);
        let res = super::part2(start, splitters);
        assert_eq!(res, 40);
    }
}