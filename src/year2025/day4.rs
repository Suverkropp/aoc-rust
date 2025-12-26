use crate::aoc::{Grid, read_grid};

pub fn handle_input(input: &str) -> Grid<bool> {
    read_grid(input).map(|x| *x == '@')
}

pub fn part1(grid: &Grid<bool>) -> usize {
    let mut count = 0;
    for i in 0..grid.get_height() {
        for j in 0..grid.get_width() {
            if grid.get(i, j).unwrap() && accessible(grid,i,j) {
                count += 1;
            }
        }
    }
    count
}

fn accessible(grid: &Grid<bool>, i: usize, j: usize) -> bool {
    count_neighbours(grid, i, j) < 4
}

fn count_neighbours(grid: &Grid<bool>, i: usize, j: usize) -> usize {
    grid.neighbours(i, j).into_iter().filter(|x| *x).count()
}

pub fn part2(grid: &mut Grid<bool>) -> usize {
    let mut count = 0;
    loop {
        let removed_now = remove_rolls(grid);
        count += removed_now;
        if removed_now == 0 {
            break count;
        }
    }
}

fn remove_rolls(grid: &mut Grid<bool>) -> usize {
    let mut count = 0;
    for i in 0..grid.get_height() {
        for j in 0..grid.get_width() {
            if grid.get(i, j).unwrap() && accessible(grid,i,j) {
                grid.set(i, j, false);
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc::get_test_input;

    #[test]
    pub fn part1(){
        let input = get_test_input(2025, 4);
        let input = handle_input(&input);
        let res = super::part1(&input);
        assert_eq!(res, 13);
    }

    #[test]
    pub fn part2(){
        let input = get_test_input(2025, 4);
        let mut input = handle_input(&input);
        let res = super::part2(&mut input);
        assert_eq!(res, 43);
    }
}
