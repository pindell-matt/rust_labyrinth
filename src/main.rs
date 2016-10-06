extern crate rand;
mod generate;
mod solve;
mod print;

use rand::{thread_rng, Rng};
use print::print_to_console;
use solve::generate_solution;

static SIZE: usize = 4;

fn main() {
    let maze = generate_maze();

    let unwalked_maze = reset_maze(maze);
    print_to_console(&unwalked_maze);

    let result = generate_solution(&unwalked_maze);
    print_to_console(&result);
}

fn reset_maze(maze: Vec<Vec<Vec<i32>>>) -> Vec<Vec<Vec<i32>>> {
    let mut solution = vec![];
    for row in maze {
        let mut new_row = vec![];
        for cell in row {
            let mut clone_cell = cell.clone();
            clone_cell[4] = 0;
            new_row.push(clone_cell);
        }
        solution.push(new_row);
    }

    solution[0][0][4] = 2;
    solution[SIZE - 1][SIZE - 1][3] = 1;

    solution
}

fn generate_maze() -> Vec<Vec<Vec<i32>>> {
    let mut grid = generate_grid();
    let mut r = 0;
    let mut c = 0;
    let mut history = vec![(r, c)];

    while history.len() != 0 {
        grid[r][c][4] = 1;

        let check = populate_check(r, c, &grid);

        if check.len() != 0 {
            history.push((r, c));

            let ref move_direction = shuffle_contents(check)[0];

            if are_equal(move_direction, 'L') {
                grid[r][c][0] = 1;
                c -= 1;
                grid[r][c][2] = 1;
            }

            if are_equal(move_direction, 'U') {
                grid[r][c][1] = 1;
                r -= 1;
                grid[r][c][3] = 1;
            }

            if are_equal(move_direction, 'R') {
                grid[r][c][2] = 1;
                c += 1;
                grid[r][c][0] = 1;
            }

            if are_equal(move_direction, 'D') {
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

    grid
}

fn generate_grid() -> Vec<Vec<Vec<i32>>> {
    let cell = vec![0; 5];
    let row: Vec<Vec<i32>> = vec![cell; SIZE];
    let grid: Vec<Vec<Vec<i32>>> = vec![row; SIZE];
    grid
}

fn populate_check(r: usize, c: usize, grid: &Vec<Vec<Vec<i32>>>) -> Vec<String> {
    let mut check = vec![];
    if c > 0 && grid[r][c - 1][4] == 0 { check.push('L'.to_string()); };
    if r > 0 && grid[r - 1][c][4] == 0 { check.push('U'.to_string()); };
    if c < (SIZE - 1) && grid[r][c + 1][4] == 0 { check.push('R'.to_string()); };
    if r < (SIZE - 1) && grid[r + 1][c][4] == 0 { check.push('D'.to_string()); };
    check
}

fn shuffle_contents(check: Vec<String>) -> Vec<String> {
    let mut cloned = check.clone();
    let mut rng = thread_rng();
    rng.shuffle(&mut cloned);
    cloned
}

fn are_equal(move_direction: &String, direction: char) -> bool {
    move_direction.to_string() == direction.to_string()
}

#[cfg(test)]
mod tests {
    use super::generate_grid;
    use super::are_equal;

    #[test]
    fn can_generate_grid_of_nested_vecs() {
        let grid = generate_grid();
        assert_eq!(4, grid.len());
        assert_eq!(4, grid[0].len());
    }

    #[test]
    fn cells_are_five_element_vecs_defaulting_to_zero() {
        let grid = generate_grid();
        assert_eq!(5, grid[0][0].len());
        assert_eq!(0, grid[0][0][0]);
        assert_eq!(0, grid[0][0][1]);
        assert_eq!(0, grid[0][0][2]);
        assert_eq!(0, grid[0][0][3]);
    }

    #[test]
    fn returns_true_if_equal() {
        let string = "L".to_string();
        assert!(are_equal(&string, 'L'));
        assert_eq!(false, are_equal(&string, 'R'));
    }

}
