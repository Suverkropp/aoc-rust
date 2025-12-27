use std::cmp::{max, min};
use itertools::Itertools;

pub struct Position {
    x: i64,
    y: i64,
    z: i64,
}

pub fn handle_input(input: &str) -> Vec<Position> {
    input.lines().map(read_position).collect()
}

fn read_position(line: &str) -> Position {
    let mut nums = line.split(',');
    let x = nums.next().unwrap().parse().unwrap();
    let y = nums.next().unwrap().parse().unwrap();
    let z = nums.next().unwrap().parse().unwrap();
    Position { x, y, z }
}

pub fn part1(junction_boxes: Vec<Position>, connections: i32) -> i32 {
    let mut circuits = Vec::from_iter(0..junction_boxes.len());
    let distances = pairs_by_distance(&junction_boxes);

    for (jbox1, jbox2) in distances.iter().take(connections as usize) {
        connect_circuits(jbox1, jbox2, &mut circuits);
    }
    let mut sizes = circuit_sizes(&circuits);
    sizes.sort();
    sizes.iter().rev().take(3).product()
}

fn circuit_sizes(circuits: &[usize]) -> Vec<i32> {
    let mut sizes = vec![0; circuits.len()];
    for junction_box in circuits {
        let mut circuit = *junction_box;
        while circuits[circuit] != circuit {
            circuit = circuits[circuit];
        }
        sizes[circuit] += 1;
    }
    sizes
}

fn connect_circuits(jbox1: &usize, jbox2: &usize, circuits: &mut Vec<usize>) {
    let mut circuit_name = *min(jbox1, jbox2);
    let mut to_update = *max(jbox1, jbox2);
    // In the loop we have circuit_name < to_update
    while circuits[to_update] != to_update {
        let pointed_to = circuits[to_update];
        if pointed_to < circuit_name {
            to_update = circuit_name;
            circuit_name = pointed_to;
        } else if pointed_to == circuit_name {
            break;
        } else {
            to_update = pointed_to;
        }
    }
    circuits[to_update] = circuit_name;
}

fn pairs_by_distance(junction_boxes: &Vec<Position>) -> Vec<(usize, usize)> {
    let mut distances: Vec<_> = pairs(0, junction_boxes.len())
        .map(|(i, j)| ((i, j), dist(&junction_boxes[i], &junction_boxes[j])))
        .collect();
    distances.sort_by(|(_, d1), (_, d2)| d1.total_cmp(d2));
    distances.iter().map(|(pair, _)| *pair).collect()
}

fn pairs(from: usize, to: usize) -> impl Iterator<Item = (usize, usize)> {
    (from..to).tuple_combinations()
}

fn dist(p1: &Position, p2: &Position) -> f64 {
    f64::sqrt(((p1.x - p2.x).pow(2) + (p1.y - p2.y).pow(2) + (p1.z - p2.z).pow(2)) as f64)
}

pub fn part2(junction_boxes: Vec<Position>) -> i64 {
    let pairs = pairs_by_distance(&junction_boxes);
    let mut circuits = Vec::from_iter(0..junction_boxes.len());
    let mut res = None;
    for (i, j) in pairs {
        connect_circuits(&i, &j, &mut circuits);
        if all_connected(&mut circuits){
            res = Some(junction_boxes[i].x * junction_boxes[j].x);
            break;
        }
    }
    res.unwrap_or_else(|| panic!())
}

fn all_connected(circuits: &mut Vec<usize>) -> bool {
    while let Some(i) = circuits.iter().rposition(|i| *i != 0) {
        set_circuit(circuits, &i);
        if circuits[i] != 0 {
            return false;
        }
    }
    true
}

fn set_circuit(circuits: &mut Vec<usize>, i: &usize) {
    while circuits[circuits[*i]] != circuits[*i] {
        circuits[*i] = circuits[circuits[*i]];
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::get_test_input;

    #[test]
    pub fn distances() {
        let p1 = Position { x: 0, y: 0, z: 0 };
        let p2 = Position { x: 0, y: 2, z: 1 };
        assert_eq!(dist(&p1, &p2), f64::sqrt(5.0));
        assert_eq!(dist(&p2, &p1), f64::sqrt(5.0));
    }

    #[test]
    pub fn comparisons() {
        let positions = vec![
            Position { x: 1, y: 1, z: 1 },
            Position { x: 1, y: 1, z: 3 },
            Position { x: 1, y: 2, z: 3 },
        ];
        let distances = pairs_by_distance(&positions);
        let expected = vec![(1, 2), (0, 1), (0, 2)];
        assert_eq!(distances, expected);
    }

    #[test]
    pub fn part1() {
        let input = get_test_input(2025, 8);
        let junction_boxes = handle_input(&input);
        let res = super::part1(junction_boxes, 10);
        assert_eq!(res, 40);
    }

    #[test]
    pub fn part2() {
        let input = get_test_input(2025, 8);
        let junction_boxes = handle_input(&input);
        let res = super::part2(junction_boxes);
        assert_eq!(res, 25272);
    }
}
