pub fn handle_input(input: &str) -> impl Iterator<Item = (u64, u64)> {
    input.split(',').map(|range| {
        let (begin, end) = range.split_once('-').unwrap();
        let begin: u64 = begin.parse::<u64>().unwrap();
        let end: u64 = end.parse::<u64>().unwrap();
        (begin, end)
    })
}

pub fn part1(ranges: impl Iterator<Item = (u64, u64)>) -> u64 {
    ranges
        .flat_map(|(begin, end)| begin..end + 1)
        .filter(|id| is_double(*id))
        .sum()
}

fn is_double(id: u64) -> bool {
    let id = id.to_string();
    if id.len() % 2 != 0 {
        return false;
    }
    let (begin, end) = id.split_at(id.len() / 2);
    begin == end
}

pub fn part2(ranges: impl Iterator<Item = (u64, u64)>) -> u64 {
    ranges
        .flat_map(|(begin, end)| begin..end + 1)
        .filter(|id| is_repeated(*id))
        .sum()
}

fn is_repeated(id: u64) -> bool {
    let id = id.to_string();
    let len = id.len();
    for i in 1..len/2+1 {
        let seg = id.split_at(i).0;
        if id == seg.repeat(len/i) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc::get_test_input;

    #[test]
    pub fn part1(){
        let input = get_test_input(2025, 2);
        let input = handle_input(&input);
        let res = super::part1(input);
        assert_eq!(res, 1227775554);
    }

    #[test]
    pub fn part2(){
        let input = get_test_input(2025, 2);
        let input = handle_input(&input);
        let res = super::part2(input);
        assert_eq!(res, 4174379265);
    }

}