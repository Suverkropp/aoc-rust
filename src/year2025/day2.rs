pub fn handle_input(input: &String) -> impl Iterator<Item = (u64, u64)> {
    input.split(',')
        .map(|range| {
        let (begin, end) = range.split_once('-').unwrap();
        let begin:u64 = begin.parse::<u64>().unwrap();
        let end :u64 = end.parse::<u64>().unwrap();
        (begin, end)
    })
}

pub fn part1(ranges : impl Iterator<Item=(u64, u64)>) -> u64 {
    ranges.flat_map(|(begin, end)| { begin..end+1 })
        .filter(|id| is_double(*id))
        .sum()
}

fn is_double(id: u64) -> bool {
    let id = id.to_string();
    if id.len() % 2 != 0 {
        return false
    }
    let (begin, end) = id.split_at(id.len() / 2);
    begin == end
}
