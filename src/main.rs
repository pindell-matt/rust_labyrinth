extern crate rand;
mod generate;
mod solve;
mod print;

use rand::{thread_rng, Rng};
use print::print_to_console;

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

fn generate_solution(grid: &Vec<Vec<Vec<i32>>>) -> Vec<Vec<Vec<i32>>>{
    let src = (0, 0);
    let dst = ((SIZE - 1) as i32, (SIZE - 1) as i32);

    let mut solution = grid.clone();
    let mut best_md = manhattan_dist(src, dst);
    let mut current = (0, 0);
    solution[(current.0) as usize][(current.1) as usize][4] = 1;

    while !cells_match(current, dst) {
        if !productive_path(&solution, current, best_md, dst).is_empty() {
            let path = productive_path(&solution, current, best_md, dst);
            current = path[0].1;
            best_md = manhattan_dist(current, dst);
            solution[(current.0) as usize][(current.1) as usize][4] = 1;
        } else {
            let original = current;
            current = desperate_path(&solution, current);
            solution[(current.0) as usize][(current.1) as usize][4] = 1;

            // move to previous position if stuck
            if current == desperate_path(&solution, current) { current = original; }
            if original == desperate_path(&solution, current) { break; }
        }
    }
    solution
}

fn desperate_path(grid: &Vec<Vec<Vec<i32>>>, current: (i32, i32)) -> (i32, i32) {
    let data = cell_data(&grid, current);
    let left  = (current.0, current.1 - 1);
    let right = (current.0, current.1 + 1);
    let up    = (current.0 - 1, current.1);
    let down  = (current.0 + 1, current.1);

    if data[2] == 1 && grid[(right.0) as usize][(right.1) as usize][4] == 0 { return right };
    if data[3] == 1 && grid[(down.0) as usize][(down.1) as usize][4]   == 0 { return down };
    if data[0] == 1 && grid[(left.0) as usize][(left.1) as usize][4]   == 0 { return left };
    if data[1] == 1 && grid[(up.0) as usize][(up.1) as usize][4]       == 0 { return up };

    current
}

fn productive_path(grid: &Vec<Vec<Vec<i32>>>, curr: (i32, i32), best_md: i32, dst: (i32, i32)) -> Vec<(i32, (i32, i32))> {
    let curr_data = cell_data(&grid, curr);
    let mut paths = vec![];
    let left  = (curr.0, curr.1 - 1);
    let right = (curr.0, curr.1 + 1);
    let up    = (curr.0 - 1, curr.1);
    let down  = (curr.0 + 1, curr.1);

    if curr_data[0] == 1 && manhattan_dist(left, dst) <= best_md &&
        grid[(left.0) as usize][(left.1) as usize][4] == 0 { paths.push( (manhattan_dist(left, dst), left) ); }
    if curr_data[2] == 1 && manhattan_dist(right, dst) <= best_md &&
        grid[(right.0) as usize][(right.1) as usize][4] == 0 { paths.push( (manhattan_dist(right, dst), right) ); }
    if curr_data[1] == 1 && manhattan_dist(up, dst) <= best_md &&
        grid[(up.0) as usize][(up.1) as usize][4] == 0 { paths.push( (manhattan_dist(up, dst), up) ); }
    if curr_data[3] == 1 && manhattan_dist(down, dst) <= best_md &&
        grid[(down.0) as usize][(down.1) as usize][4] == 0 { paths.push( (manhattan_dist(down, dst), down) ); }

    paths.sort();
    paths
}

fn cell_data(grid: &Vec<Vec<Vec<i32>>>, coordinates: (i32, i32)) -> Vec<i32> {
    let ref data = grid[(coordinates.0) as usize][(coordinates.1) as usize];
    data.clone()
}

fn cells_match(current: (i32, i32), destination: (i32, i32)) -> bool {
    current.0 == destination.0 && current.1 == destination.1
}

fn manhattan_dist(a: (i32, i32), b: (i32, i32)) -> i32 {
    let y_val = (a.0 - b.0).abs();
    let x_val = (a.1 - b.1).abs();
    x_val + y_val
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
