use crate::grid::Grid;
use itertools::{iproduct, Itertools};
use std::cmp::{max, min, Reverse};
use std::collections::VecDeque;

pub type Position = (usize, usize);

pub fn handle_input(input: &str) -> Vec<Position> {
    input.lines().map(read_position).collect()
}

fn read_position(line: &str) -> Position {
    let (x, y) = line.split_once(',').unwrap();
    (x.parse().unwrap(), y.parse().unwrap())
}

pub fn part1(red_tiles: &Vec<Position>) -> usize {
    let (_corners, area) = rectangles_by_area(red_tiles)[0];
    area
}

fn rectangles_by_area(red_tiles: &Vec<Position>) -> Vec<((Position, Position), usize)> {
    let mut pairs: Vec<_> = red_tiles
        .iter()
        .tuple_combinations::<(_, _)>()
        .map(|(p1, p2)| ((*p1, *p2), rectangle_area(p1, p2)))
        .collect();
    pairs.sort_by_key(|(_, area)| Reverse(*area));
    pairs
}

fn rectangle_area((x1, y1): &Position, (x2, y2): &Position) -> usize {
    let x_dist = x1.abs_diff(*x2) + 1;
    let y_dist = y1.abs_diff(*y2) + 1;
    x_dist * y_dist
}

pub fn part2(red_tiles: &Vec<Position>) -> usize {
    let green_tiles = find_green_tiles(red_tiles);

    let rectangles = rectangles_by_area(red_tiles);
    rectangles
        .iter()
        .filter(|((p1, p2), _area)| in_rectangle_green(&green_tiles, *p1, *p2))
        .map(|((_, _), area)| *area)
        .next()
        .expect("No rectangles completely green")
}

fn find_green_tiles(red_tiles: &Vec<Position>) -> Grid<bool> {
    let width = red_tiles.iter().map(|(x, _)| *x).max().unwrap() + 1;
    let height = red_tiles.iter().map(|(_, y)| *y).max().unwrap() + 1;
    let lines = find_lines(red_tiles, width, height);

    in_lines(&lines)
}

fn in_lines(lines: &Grid<bool>) -> Grid<bool> {
    let width = lines.get_width();
    let height = lines.get_height();
    let mut in_lines = Grid::new(width, height, true);
    let mut visited = Grid::new(width, height, false);
    let mut queue = VecDeque::new();
    queue.push_back((0, 0));
    visited.set(0, 0, true);
    let mut counter = 0;

    while let Some((x, y)) = queue.pop_front() {
        counter += 1;
        if counter % 10_000_000 == 0 {
            println!("Counter: {}", counter);
            println!("Queue size: {}", queue.len());
        }
        if lines.get(x, y).unwrap() {
            continue;
        }
        in_lines.set(x, y, false);
        if x > 0 {
            add_to_queue(&mut queue, &mut visited, x - 1, y);
        }
        if y > 0 {
            add_to_queue(&mut queue, &mut visited, x, y - 1);
        }
        if x < width - 1 {
            add_to_queue(&mut queue, &mut visited, x + 1, y);
        }
        if y < height - 1 {
            add_to_queue(&mut queue, &mut visited, x, y + 1);
        }
    }
    println!("Tiles coloured green (or red): {counter}/{}", width*height);
    in_lines
}

fn add_to_queue(queue: &mut VecDeque<(usize, usize)>, visited: &mut Grid<bool>, x: usize, y: usize) {
    if !visited.get(x, y).unwrap() {
        visited.set(x, y, true);
        queue.push_back((x, y));
    }
}

fn find_lines(red_tiles: &Vec<Position>, width: usize, height: usize) -> Grid<bool> {
    let mut on_lines = Grid::new(width, height, false);
    let mut points = red_tiles.iter();
    let mut old_point = points.next().unwrap();
    for point in points {
        mark_line(&mut on_lines, old_point, point);
        old_point = point;
    }
    mark_line(&mut on_lines, old_point, &red_tiles[0]);
    on_lines
}

fn mark_line(grid: &mut Grid<bool>, (x1, y1): &Position, (x2, y2): &Position) {
    if x1 == x2 && y1 != y2 {
        let x = *x1;
        let y_min = *min(y1, y2);
        let y_max = *max(y1, y2);
        for y in y_min..=y_max {
            grid.set(x, y, true);
        }
    } else if x1 != x2 && y1 == y2 {
        let y = *y1;
        let x_min = *min(x1, x2);
        let x_max = *max(x1, x2);
        for x in x_min..=x_max {
            grid.set(x, y, true);
        }
    } else {
        panic!("The points ({x1}, {y1}) and ({x2}, {y2}) do not form a line");
    }
}

fn in_rectangle_green(green_tiles: &Grid<bool>, (x1, y1): Position, (x2, y2): Position) -> bool {
    let x_min = min(x1, x2);
    let x_max = max(x1, x2);
    let y_min = min(y1, y2);
    let y_max = max(y1, y2);

    iproduct!(x_min..=x_max, y_min..=y_max).all(|(x, y)| green_tiles.get(x, y).unwrap())
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

    #[test]
    pub fn part2() {
        let input = get_test_input(2025, 9);
        let input = handle_input(&input);
        let res = super::part2(&input);
        assert_eq!(res, 24);
    }
}
