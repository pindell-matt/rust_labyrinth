extern crate rand;

use rand::{thread_rng, Rng};

static SIZE: usize = 8;

fn main() {
    let maze = generate_maze();
    print_to_console(&maze);
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

    // set start
    grid[0][0][4] = 2;
    // set finish
    grid[SIZE - 1][SIZE - 1][3] = 1;

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

fn print_to_console(grid: &Vec<Vec<Vec<i32>>>) {
    println!("");
    for row in 0..grid.len() {
        if row == 0 { print_top_row(&grid[row]); };
        print_vertical_walls(&grid[row]);
        print_horizontal_walls(&grid[row]);
    }
    println!("");
}

fn print_top_row(row: &Vec<Vec<i32>>) {
    let mut top = vec!["+"];
    for cell in row {
        if cell[4] == 2 {
            top.push("   +");
        } else {
            top.push("---+");
        }
    }
    let joined = top.join("");
    println!("{:?}", joined);
}

fn print_vertical_walls(row: &Vec<Vec<i32>>) {
    let mut vertical = vec!["|"];
    for cell in row {
        vertical.push("   ");
        if cell[2] == 1 {
            vertical.push(" ");
        } else {
            vertical.push("|");
        }
    }
    let joined = vertical.join("");
    println!("{:?}", joined);
}

fn print_horizontal_walls(row: &Vec<Vec<i32>>) {
    let mut horizontal = vec!["+"];
    for cell in row {
        if cell[3] == 1 {
            horizontal.push("   +");
        } else {
            horizontal.push("---+");
        }
    }
    let joined = horizontal.join("");
    println!("{:?}", joined);
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
