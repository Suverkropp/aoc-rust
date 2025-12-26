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

pub fn part2(_grid: &mut Grid<bool>) -> usize {


    0
}

// fn remove_rolls(grid: &mut Grid<bool>) {
//     grid.index_iter().for_each(
//         |(i, j)| {
//             let new_value = grid.get(i,j).unwrap() && !accessible(grid, i, j);
//             grid.set(i,j,new_value);
//         }
//     );
// }