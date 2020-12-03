use std::fs;

fn solve_slope(file: String, slope: i32, down: i32) -> i32 {
    let mut tree_hits: i32 = 0;
    let tree: char = '#';
    for (line_num, line) in file.lines().enumerate() {
        if line_num % down as usize == 0 {
            let line_length = line.chars().count();
            let position = (line_num / down as usize * slope as usize).rem_euclid(line_length);

            if line.chars().nth(position).unwrap() == tree {
                tree_hits = tree_hits + 1
            }
        }
    }

    return tree_hits;
}
pub fn solve_a(filename: &str) {
    let file: String = fs::read_to_string(filename).expect("Error opening file");
    let tree_hits = solve_slope(file, 3, 1);
    println!("{}", tree_hits);
}

pub fn solve_b(filename: &str) {
    let file: String = fs::read_to_string(filename).expect("Error opening file");
    // Conver to i128 as result is too large for i32
    let tree_hits_1 = solve_slope(file.clone(), 1, 1) as i128;
    let tree_hits_2 = solve_slope(file.clone(), 3, 1) as i128;
    let tree_hits_3 = solve_slope(file.clone(), 5, 1) as i128;
    let tree_hits_4 = solve_slope(file.clone(), 7, 1) as i128;
    let tree_hits_5 = solve_slope(file.clone(), 1, 2) as i128;
    println!(
        "{}",
        tree_hits_1 * tree_hits_2 * tree_hits_3 * tree_hits_4 * tree_hits_5
    )
}
