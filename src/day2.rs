use std::fs;
pub fn solve_a(filename: &str) {
    let foo: String = fs::read_to_string(filename).expect("Error opening file");
    let mut valid_passwords: i32 = 0;
    for line in foo.lines() {
        let line_parts: Vec<&str> = line.split_whitespace().collect();
        let range = line_parts[0];
        let character = line_parts[1].replace(":", "");
        let password = line_parts[2];
        let bounds: Vec<i32> = range
            .split("-")
            .map(|n| n.parse().expect("Cannot parse range"))
            .collect();
        let number: i32 = password.matches(&character).count() as i32;
        if (number >= bounds[0]) && (number <= bounds[1]) {
            valid_passwords = valid_passwords + 1;
        }
    }
    println!("Total valid passwords: {}", valid_passwords);
}
