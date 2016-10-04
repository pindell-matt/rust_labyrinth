extern crate rand;

use rand::{thread_rng, Rng};

static SIZE: usize = 8;

fn main() {
    let mut grid = create_grid();
    let mut r = 0;
    let mut c = 0;
    let mut history = vec![(r, c)];

    while history.len() != 0 {
        grid[r][c][4] = 1;

        let mut check = vec![];
        if c > 0 && grid[r][c - 1][4] == 0 { check.push('L'); };
        if r > 0 && grid[r - 1][c][4] == 0 { check.push('U'); };

        if c < (SIZE - 1) && grid[r][c + 1][4] == 0 { check.push('R'); };
        if r < (SIZE - 1) && grid[r + 1][c][4] == 0 { check.push('D'); };

        if check.len() != 0 {
            history.push((r, c));

            let mut rng = thread_rng();
            rng.shuffle(&mut check);
            let move_direction = check[0];

            if move_direction == 'L' {
                grid[r][c][0] = 1;
                c -= 1;
                grid[r][c][2] = 1;
            }

            if move_direction == 'U' {
                grid[r][c][1] = 1;
                r -= 1;
                grid[r][c][3] = 1;
            }

            if move_direction == 'R' {
                grid[r][c][2] = 1;
                c += 1;
                grid[r][c][0] = 1;
            }

            if move_direction == 'D' {
                grid[r][c][3] = 1;
                r += 1;
                grid[r][c][1] = 1;
            }
        } else {
            let popped = history.pop().unwrap();
            r = popped.0;
            c = popped.1;
        }
    }
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