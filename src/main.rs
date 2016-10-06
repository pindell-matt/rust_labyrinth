mod generate;
mod reset;
mod solve;
mod print;

use generate::generate_maze;
use reset::reset_maze;
use print::print_to_console;
use solve::generate_solution;

static SIZE: usize = 4;

fn main() {
    let maze = generate_maze(SIZE);

    let unwalked_maze = reset_maze(maze);
    print_to_console(&unwalked_maze);

    let result = generate_solution(&unwalked_maze);
    print_to_console(&result);
}

// fn reset_maze(maze: Vec<Vec<Vec<i32>>>) -> Vec<Vec<Vec<i32>>> {
//     let mut solution = vec![];
//     for row in maze {
//         let mut new_row = vec![];
//         for cell in row {
//             let mut clone_cell = cell.clone();
//             clone_cell[4] = 0;
//             new_row.push(clone_cell);
//         }
//         solution.push(new_row);
//     }
//
//     // set start and exit
//     solution[0][0][4] = 2;
//     solution[SIZE - 1][SIZE - 1][3] = 1;
//
//     solution
// }
