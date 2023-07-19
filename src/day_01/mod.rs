use std::fs;

pub fn solve() {
    println!("Solving Day 1 - Part 1");
    solve_1();

    println!("Solving Day 1 - Part 2");
    solve_2();
}

fn solve_1() {
    let file_path = "src/day_01/input.txt";
    let input = fs::read_to_string(file_path).expect("Can't read input file.");

    let mut floor: isize = 0;
    for char in input.chars() {
        match char {
            ')' => floor -= 1,
            '(' => floor += 1,
            '\n' => {}
            char => {
                eprintln!("Unexpected character in input string.");
                eprintln!("{}", char as u32);
            }
        }
    }
    println!("Final floor: {}", floor);
}

fn solve_2() {
    let file_path = "src/day_01/input.txt";
    let input = fs::read_to_string(file_path).expect("Can't read input file.");

    let mut floor: isize = 0;
    for (position, char) in input.chars().enumerate() {
        match char {
            ')' => floor -= 1,
            '(' => floor += 1,
            '\n' => {}
            char => {
                eprintln!("Unexpected character in input string.");
                eprintln!("{}", char as u32);
            }
        }
        if floor == -1 {
            println!("Entering basement on char: {}", position + 1);
            return;
        }
    }
}
