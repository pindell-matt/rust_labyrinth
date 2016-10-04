extern crate rand;

use rand::{thread_rng, Rng};

static SIZE: usize = 8;

fn main() {
    let mut grid = create_grid();
    println!("{:?}", grid);
}

fn create_grid() -> Vec<Vec<Vec<i32>>> {
    let cell = vec![0; 5];
    let row: Vec<Vec<i32>> = vec![cell; SIZE];
    let grid: Vec<Vec<Vec<i32>>> = vec![row; SIZE];
    return grid
}

#[cfg(test)]
mod tests {
    use super::create_grid;

    #[test]
    fn can_create_grid_of_nested_vecs() {
        let grid = create_grid();
        assert_eq!(8, grid.len());
        assert_eq!(8, grid[0].len());
    }
}
