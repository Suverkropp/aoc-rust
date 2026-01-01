use std::str::Lines;

type Present = [[bool; 3]; 3];

type Region = ((usize, usize), [u32; 6]);

pub fn handle_input(input: &str) -> ([Present; 6], Vec<Region>) {
    let mut lines = input.lines();
    let mut presents = Vec::new();
    for i in 0..6 {
        presents.push(parse_present(&mut lines, i));
    }
    let presents: [Present; 6] = presents.try_into().unwrap();

    let regions = lines.map(parse_region).collect();
    (presents, regions)
}

fn parse_present(lines: &mut Lines, i: usize) -> Present {
    let mut present = [[false; 3]; 3];
    assert_eq!(lines.next().unwrap(), format!("{i}:"));
    for y in 0..3 {
        let line = lines.next().unwrap();
        let mut line = line.chars();
        for x in 0..3 {
            let val = match line.next() {
                Some('.') => false,
                Some('#') => true,
                Some(c) => panic!("Invalid character '{c}' at position ({x},{y}) in present {i}"),
                None => panic!("Missing character at position ({x},{y}) in present {i}"),
            };
            present[x][y] = val;
        }
    }
    assert_eq!(lines.next().unwrap(), "");
    present
}

fn parse_region(line: &str) -> Region {
    let (dims, presents) = line.split_once(':').unwrap();
    let (x, y) = dims.split_once('x').unwrap();
    let (x, y) = (x.parse().unwrap(), y.parse().unwrap());
    let presents = presents
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<u32>>()
        .try_into()
        .unwrap();
    ((x, y), presents)
}

pub fn part1(presents: &[Present; 6], regions: &Vec<Region>) -> usize {
    regions.iter().filter(|region| presents_fit(region, presents)).count()
}

fn presents_fit(region: &Region, presents: &[Present; 6]) -> bool {
    let ((x, y), numbers) = region;
    let total_presents: u32 = numbers.iter().sum();
    let space_per_present: [u32; 6] = presents.map(|p| p.iter().map(|line| {
        (if line[0] { 1 } else { 0 })
            + if line[1] { 1 } else { 0 }
            + if line[2] { 1 } else { 0 }
    }).sum());
    let mut total_spaces = 0;
    for i in 0..6 {
        total_spaces += space_per_present[i] * numbers[i];
    }

    if (x/3)*(y/3) >= total_presents as usize {
        return true
    }
    if total_spaces as usize > x * y {
        return false
    }
    panic!("This one is actually hard...")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::get_test_input;

    #[test]
    pub fn part1() {
        let input = get_test_input(2025, 12);
        let (presents, regions) = handle_input(&input);
        let res = super::part1(&presents, &regions);
        assert_eq!(res, 2);
    }
}
