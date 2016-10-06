pub fn print_to_console(grid: &Vec<Vec<Vec<i32>>>) {
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
        if cell == &row[0] {
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
        if cell[4] == 1 { vertical.push(" * "); } else { vertical.push("   "); }
        if cell[2] == 1 { vertical.push(" "); } else { vertical.push("|"); }
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
