use crate::grid::Grid;
use crate::year2025::day9::Tile::{Green, Red, White};
use itertools::{Itertools, iproduct};
use std::cmp::{PartialEq, Reverse, max, min};
use std::collections::VecDeque;

pub type Position = (usize, usize);

#[derive(Copy, Clone, Eq, PartialEq)]
enum Tile {
    Red,
    Green,
    White,
}

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
    let mut counter: u64 = 0;
    rectangles
        .iter()
        .filter(|((p1, p2), area)| {
            counter += 1;
            if counter % 1_000 == 0 {
                println!("Counter: {counter}");
                println!("Area: {area}");
            }
            no_white_in_rectangle(&green_tiles, *p1, *p2)
        })
        .map(|((_, _), area)| *area)
        .next()
        .expect("No rectangles completely green")
}

fn find_green_tiles(red_tiles: &Vec<Position>) -> Grid<Tile> {
    let width = red_tiles.iter().map(|(x, _)| *x).max().unwrap() + 2;
    let height = red_tiles.iter().map(|(_, y)| *y).max().unwrap() + 2;
    let mut tiles = colour_lines_red(red_tiles, width, height);
    colour_white_outside_red(&mut tiles);
    tiles
}

fn colour_white_outside_red(tiles: &mut Grid<Tile>) {
    let width = tiles.get_width();
    let height = tiles.get_height();
    let mut queue = VecDeque::new();
    queue.push_back((0, 0));
    tiles.set(0, 0, Green);
    let mut counter: u64 = 0;

    while let Some((x, y)) = queue.pop_front() {
        counter += 1;
        if counter % 100_000_000 == 0 {
            println!("Counter: {}", counter);
            println!("Queue size: {}", queue.len());
        }
        if x > 0 {
            add_to_queue(&mut queue, tiles, x - 1, y);
        }
        if y > 0 {
            add_to_queue(&mut queue, tiles, x, y - 1);
        }
        if x < width - 1 {
            add_to_queue(&mut queue, tiles, x + 1, y);
        }
        if y < height - 1 {
            add_to_queue(&mut queue, tiles, x, y + 1);
        }
    }
    println!("Tiles coloured white: {counter}/{}", width * height);
}

fn add_to_queue(
    queue: &mut VecDeque<(usize, usize)>,
    tiles: &mut Grid<Tile>,
    x: usize,
    y: usize,
) {
    if tiles.get(x, y).unwrap() == Green {
        tiles.set(x, y, White);
        queue.push_back((x, y));
    }
}

fn colour_lines_red(red_tiles: &Vec<Position>, width: usize, height: usize) -> Grid<Tile> {
    let mut on_lines = Grid::new(width, height, Green);
    let mut points = red_tiles.iter();
    let mut old_point = points.next().unwrap();
    for point in points {
        colour_line_red(&mut on_lines, old_point, point);
        old_point = point;
    }
    colour_line_red(&mut on_lines, old_point, &red_tiles[0]);
    on_lines
}

fn colour_line_red(grid: &mut Grid<Tile>, (x1, y1): &Position, (x2, y2): &Position) {
    if x1 == x2 && y1 != y2 {
        let x = *x1;
        let y_min = *min(y1, y2);
        let y_max = *max(y1, y2);
        for y in y_min..=y_max {
            grid.set(x, y, Red);
        }
    } else if x1 != x2 && y1 == y2 {
        let y = *y1;
        let x_min = *min(x1, x2);
        let x_max = *max(x1, x2);
        for x in x_min..=x_max {
            grid.set(x, y, Red);
        }
    } else {
        panic!("The points ({x1}, {y1}) and ({x2}, {y2}) do not form a line");
    }
}

fn no_white_in_rectangle(green_tiles: &Grid<Tile>, (x1, y1): Position, (x2, y2): Position) -> bool {
    let x_min = min(x1, x2);
    let x_max = max(x1, x2);
    let y_min = min(y1, y2);
    let y_max = max(y1, y2);

    iproduct!(x_min..=x_max, y_min..=y_max).all(|(x, y)| green_tiles.get(x, y).unwrap() != White)
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
