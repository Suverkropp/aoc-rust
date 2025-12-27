use itertools::Itertools;

pub type Position = (i64, i64);

pub fn handle_input(input: &str) -> Vec<Position> {
    input.lines().map(read_position).collect()
}

fn read_position(line: &str) -> Position {
    let (x, y) = line.split_once(',').unwrap();
    (x.parse().unwrap(), y.parse().unwrap())
}

pub fn part1(input: &Vec<Position>) -> i64 {
    input
        .iter()
        .tuple_combinations::<(_, _)>()
        .map(|(p1, p2)| square_size(p1, p2))
        .max()
        .unwrap()
}

fn square_size((x1, y1): &Position, (x2, y2): &Position) -> i64 {
    let x_dist = (x1 - x2).abs() + 1;
    let y_dist = (y1 - y2).abs() + 1;
    x_dist * y_dist
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::get_test_input;

    #[test]
    pub fn part1() {
        let input = get_test_input(2025, 9);
        let input = handle_input(&input);
        let res = super::part1(&input);
        assert_eq!(res, 50);
    }
}
