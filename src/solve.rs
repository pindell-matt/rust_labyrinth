pub fn generate_solution(grid: &Vec<Vec<Vec<i32>>>) -> Vec<Vec<Vec<i32>>>{
    let src = (0, 0);
    let dst = ((grid.len() - 1) as i32, (grid[0].len() - 1) as i32);

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
            grid[(left.0) as usize][(left.1) as usize][4] == 0 {
                paths.push( (manhattan_dist(left, dst), left) );
            }
        if curr_data[2] == 1 && manhattan_dist(right, dst) <= best_md &&
            grid[(right.0) as usize][(right.1) as usize][4] == 0 {
                paths.push( (manhattan_dist(right, dst), right) );
            }
        if curr_data[1] == 1 && manhattan_dist(up, dst) <= best_md &&
            grid[(up.0) as usize][(up.1) as usize][4] == 0 {
                paths.push( (manhattan_dist(up, dst), up) );
            }
        if curr_data[3] == 1 && manhattan_dist(down, dst) <= best_md &&
            grid[(down.0) as usize][(down.1) as usize][4] == 0 {
                paths.push( (manhattan_dist(down, dst), down) );
            }

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
