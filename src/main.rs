mod generate;
mod reset;
mod solve;
mod print;

use generate::generate_maze;
use reset::reset_maze;
use print::print_to_console;
use solve::generate_solution;

fn main() {
    let maze = generate_maze(4);

    let unwalked_maze = reset_maze(maze);
    print_to_console(&unwalked_maze);

    let result = generate_solution(&unwalked_maze);
    print_to_console(&result);
}
