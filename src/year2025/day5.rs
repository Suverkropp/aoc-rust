use std::cmp::{max, min};
use std::collections::BTreeMap;

type Id = u64;

pub fn handle_input(input: &str) -> (Vec<(Id, Id)>, Vec<Id>) {
    let mut lines = input.lines();
    let ranges = lines.by_ref().map_while(read_range).collect();
    let ids = lines.map(|x| x.parse::<Id>().unwrap()).collect();
    (ranges, ids)
}

fn read_range(range: &str) -> Option<(Id, Id)> {
    let res = range.split_once('-');
    res.map(|(low, high)| (low.parse().unwrap(), high.parse().unwrap()))
}

pub fn part1(ranges: &Vec<(Id, Id)>, ids: &Vec<Id>) -> usize {
    ids.iter().filter(|id| is_fresh(ranges, **id)).count()
}

fn is_fresh(ranges: &Vec<(Id, Id)>, id: Id) -> bool {
    ranges.iter().any(|&(begin, end)| begin <= id && id <= end)
}

pub fn part2(ranges: &Vec<(Id, Id)>) -> u64 {
    let mut fresh_lows: BTreeMap<Id, Id> = BTreeMap::new();
    let mut fresh_highs: BTreeMap<Id, Id> = BTreeMap::new();
    for (low, high) in ranges {
        let mut new_low = *low;
        let mut new_high = *high;
        let lows_to_remove = ranges_between(&fresh_lows, *low, *high);
        for (low, high) in lows_to_remove {
            new_high = max(new_high, high);
            fresh_lows.remove(&low);
            fresh_highs.remove(&high);
        }
        let highs_to_remove = ranges_between(&fresh_highs, *low, *high);
        for (high, low) in highs_to_remove {
            new_low = min(new_low, low);
            fresh_lows.remove(&low);
            fresh_highs.remove(&high);
        }
        fresh_lows.insert(new_low, new_high);
        fresh_highs.insert(new_high, new_low);
    }
    total_range(fresh_lows)
}

fn ranges_between(ranges: &BTreeMap<Id, Id>, low: Id, high: Id) -> Vec<(Id, Id)> {
    ranges
        .range(low..=high)
        .map(|(l, h)| (*l, *h))
        .collect::<Vec<(Id, Id)>>()
}

fn total_range(ranges: BTreeMap<Id, Id>) -> u64 {
    ranges.iter().map(|(low, high)| high - low + 1).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc::get_test_input;

    #[test]
    pub fn part1() {
        let input = get_test_input(2025, 5);
        let (ranges, ids) = handle_input(&input);
        let res = super::part1(&ranges, &ids);
        assert_eq!(res, 3);
    }

    #[test]
    pub fn part2() {
        let input = get_test_input(2025, 5);
        let (ranges, _ids) = handle_input(&input);
        let res = super::part2(&ranges);
        assert_eq!(res, 14);
    }
}
