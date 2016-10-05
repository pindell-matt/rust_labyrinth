extern crate rand;

use rand::{thread_rng, Rng};

static SIZE: usize = 8;

fn main() {
    let maze = generate_maze();
    print_to_console(&maze);
    generate_solution(&maze);
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

fn generate_solution(grid: &Vec<Vec<Vec<i32>>>) {
    let src = (0, 0);
    let dst = ((SIZE - 1) as i32, (SIZE - 1) as i32);
    let mut best_MD = calc_MD(src, dst);
    let ref mut current = (src.0, src.1);
    while !cells_match(grid, current, dst) {
        if productive_path_available(current, cell_data(grid, current), best_MD, dst) {
            println!("{:?}", most_productive_path(current, cell_data(grid, current), best_MD, dst));
            // take productive path
            // update best_MD
            // update current
            break;
        } else {
            // best_MD = calc_MD(current, dst);
            break;
        }
    }
}

// L U R D
// 0 1 2 3

fn most_productive_path(curr: &(i32, i32), curr_data: Vec<i32>, best_MD: i32, dst: (i32, i32)) -> Vec<i32> {
    let mut paths = vec![];
    if curr_data[0] == 1 && calc_MD((curr.0 - 1, curr.1), dst) < best_MD { paths.push(calc_MD((curr.0 - 1, curr.1), dst)); }
    if curr_data[1] == 1 && calc_MD((curr.0, curr.1 - 1), dst) < best_MD { paths.push(calc_MD((curr.0, curr.1 - 1), dst)); }
    if curr_data[2] == 1 && calc_MD((curr.0 + 1, curr.1), dst) < best_MD { paths.push(calc_MD((curr.0 + 1, curr.1), dst)); }
    if curr_data[3] == 1 && calc_MD((curr.0, curr.1 + 1), dst) < best_MD { paths.push(calc_MD((curr.0, curr.1 + 1), dst)); }
    paths.sort();
    paths
}

fn productive_path_available(curr: &(i32, i32), curr_data: Vec<i32>, best_MD: i32, dst: (i32, i32)) -> bool {
    if curr_data[0] == 1 && calc_MD((curr.0 - 1, curr.1), dst) < best_MD { return true; }
    if curr_data[1] == 1 && calc_MD((curr.0, curr.1 - 1), dst) < best_MD { return true; }
    if curr_data[2] == 1 && calc_MD((curr.0 + 1, curr.1), dst) < best_MD { return true; }
    if curr_data[3] == 1 && calc_MD((curr.0, curr.1 + 1), dst) < best_MD { return true; }
    false
}

fn cell_data(grid: &Vec<Vec<Vec<i32>>>, coordinates: &(i32, i32)) -> Vec<i32> {
    let ref data = grid[(coordinates.0) as usize][(coordinates.1) as usize];
    data.clone()
}

fn cells_match(grid: &Vec<Vec<Vec<i32>>>, curr: &(i32, i32), dst: (i32, i32)) -> bool {
    let ref current = grid[(curr.0) as usize][(curr.1) as usize];
    let ref destination = grid[(dst.0) as usize][(dst.1) as usize];
    current == destination
}

fn calc_MD(a: (i32, i32), b: (i32, i32)) -> i32 {
    let x_val = (a.0 - b.0).abs();
    let y_val = (a.1 - b.1).abs();
    x_val + y_val
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
    use super::generate_grid;
    use super::are_equal;

    #[test]
    fn can_generate_grid_of_nested_vecs() {
        let grid = generate_grid();
        assert_eq!(8, grid.len());
        assert_eq!(8, grid[0].len());
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
