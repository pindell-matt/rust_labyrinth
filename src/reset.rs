pub fn reset_maze(maze: Vec<Vec<Vec<i32>>>) -> Vec<Vec<Vec<i32>>> {
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
    set_start_and_exit(solution)
}

fn set_start_and_exit(mut maze: Vec<Vec<Vec<i32>>>) -> Vec<Vec<Vec<i32>>> {
    let height = maze.len();
    let width = maze[0].len();

    maze[0][0][4] = 2;
    maze[height - 1][width - 1][3] = 1;
    maze
}
