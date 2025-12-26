use std::fs;
use std::iter::repeat;
use std::slice::{Iter, IterMut};

pub fn get_input(year: i32, day: i32) -> String {
    let path = format!("input/{year}/day{day}.txt");
    fs::read_to_string(path).expect("No input file")
}

pub fn get_test_input(year: i32, day: i32) -> String {
    let path = format!("input/{year}/test{day}.txt");
    fs::read_to_string(path).expect("No input file")
}

pub struct Grid<T: Copy> {
    grid: Vec<T>,
    width: usize,
    height: usize,
}

pub fn read_grid(input: &str) -> Grid<char> {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let grid: Vec<char> = input.lines().flat_map(|l| l.chars()).collect();
    Grid {
        grid,
        width,
        height,
    }
}

impl<T: Copy> Grid<T> {
    pub fn get_width(&self) -> usize { self.width }
    pub fn get_height(&self) -> usize { self.height }

    pub fn get(&self, x: usize, y: usize) -> Option<T> {
        if x >= self.width || y >= self.height {
            return None;
        }
        Some(self.grid[y * self.width + x])
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        if x >= self.width || y >= self.height {
            panic!("Out of bounds");
        }
        self.grid[y * self.width + x] = value;
    }

    pub fn map<U: Copy>(&self, f: impl Fn(&T) -> U) -> Grid<U> {
        let grid = self.iter().map(f).collect();
        Grid {
            grid,
            width: self.width,
            height: self.height,
        }
    }

    pub fn neighbours(&self, x: usize, y: usize) -> Vec<T> {
        let mut neighbours = Vec::new();
        if x > 0 && y > 0 {
            neighbours.push((x - 1, y - 1));
        }
        if x > 0 {
            neighbours.push((x - 1, y));
            neighbours.push((x - 1, y + 1));
        }
        if y > 0 {
            neighbours.push((x, y - 1));
            neighbours.push((x + 1, y - 1));
        }
        neighbours.push((x, y + 1));
        neighbours.push((x + 1, y));
        neighbours.push((x + 1, y + 1));
        neighbours
            .iter()
            .filter_map(|(i, j)| self.get(*i, *j))
            .collect()
    }
    pub fn iter(&'_ self) -> Iter<'_, T> {
        self.grid.iter()
    }

    pub fn iter_mut(&'_ mut self) -> IterMut<'_, T> {
        self.grid.iter_mut()
    }

    pub fn index_iter(&self) -> impl Iterator<Item = (usize, usize)> {
        (0..self.width).flat_map(|i| repeat(i).zip(0..self.height))
    }
}

impl<T: Copy> IntoIterator for Grid<T> {
    type Item = T;
    type IntoIter = <Vec<T> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.grid.into_iter()
    }
}