use std::cmp::{max, min};

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
    let mut fresh_ranges: Vec<(Id, Id)> = Vec::new();
    for (low, high) in ranges {
        let lowest_overlap = fresh_ranges
            .iter()
            .position(|(_l, h)| low <= h);
        let highest_overlap = fresh_ranges
            .iter()
            .rposition(|(l,_h)| l <= high);

        match (lowest_overlap, highest_overlap) {
            (None, None) => fresh_ranges.push((*low, *high)),
            (Some(_), None) => fresh_ranges.insert(0, (*low, *high)),
            (None, Some(_)) => fresh_ranges.push((*low, *high)),
            (Some(i), Some(j)) => {
                let (l, _) = fresh_ranges[i];
                let (_, h) = fresh_ranges[j];
                let low = min(*low, l);
                let high = max(*high, h);
                remove_between(&mut fresh_ranges, i, j);
                fresh_ranges.insert(i, (low, high))
            }
        }


    }
    total_range(fresh_ranges)
}

fn remove_between<T>(vec: &mut Vec<T>, min: usize, max: usize) {
    for _ in min..=max {
        vec.remove(min);
    }
}

fn total_range(ranges: impl IntoIterator<Item = (Id, Id)>) -> u64 {
    ranges.into_iter().map(|(low, high)| high - low + 1).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::get_test_input;

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
