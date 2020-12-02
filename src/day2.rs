use std::fs;
pub fn solve_a(filename: &str) {
    let foo: String = fs::read_to_string(filename).expect("Error opening file");
    let mut valid_passwords: i32 = 0;
    for line in foo.lines() {
        let line_parts: Vec<&str> = line.split_whitespace().collect();
        let range = line_parts[0];
        let character = line_parts[1].replace(":", "");
        let password = line_parts[2];
        let constraints: Vec<i32> = range
            .split("-")
            .map(|n| n.parse().expect("Cannot parse range"))
            .collect();
        let number: i32 = password.matches(&character).count() as i32;
        if (number >= constraints[0]) && (number <= constraints[1]) {
            valid_passwords = valid_passwords + 1;
        }
    }
    println!("Total valid passwords: {}", valid_passwords);
}

pub fn solve_b(filename: &str) {
    let foo: String = fs::read_to_string(filename).expect("Error opening file");
    let mut valid_passwords: i32 = 0;
    for line in foo.lines() {
        let line_parts: Vec<&str> = line.split_whitespace().collect();
        let range = line_parts[0];
        let character = line_parts[1].replace(":", "").chars().next().unwrap();
        let password = line_parts[2];
        let constraints: Vec<usize> = range
            .split("-")
            .map(|n| n.parse().expect("Cannot parse range"))
            .collect();

        match (
            password.chars().nth(constraints[0] - 1),
            password.chars().nth(constraints[1] - 1),
        ) {
            (Some(a), Some(b)) => {
                if ((a == character) || (b == character)) && (a != b) {
                    valid_passwords = valid_passwords + 1;
                }
            }
            _ => (),
        }
    }
    println!("Total valid passwords: {}", valid_passwords);
}
