use phf::phf_map;
use std::collections::HashMap;
use std::fs;

pub fn solve() {
    println!("Solving Day 3 - Part 1");
    solve_1();

    println!("Solving Day 3 - Part 2");
    solve_2();
}

#[derive(Hash, Eq, PartialEq, Debug)]
enum Instructions {
    Up,
    Down,
    Left,
    Right,
}

static INSTRUCTION_MAP: phf::Map<char, Instructions> = phf_map! {
    '^' => Instructions::Up,
    'v' => Instructions::Down,
    '<' => Instructions::Left,
    '>' => Instructions::Right
};

fn solve_1() {
    let delta_map = HashMap::from([
        (Instructions::Up, (0, -1)),
        (Instructions::Down, (0, 1)),
        (Instructions::Left, (-1, 0)),
        (Instructions::Right, (1, 0)),
    ]);

    let file_path = "src/day_03/input.txt";
    let input = fs::read_to_string(file_path).expect("could not read input file");
    let input = input.trim();
    let starting_node = (0, 0);

    let mut visited_nodes: Vec<(isize, isize)> = vec![];
    visited_nodes.push(starting_node.clone());

    let mut current_node = starting_node.clone();

    for char in input.chars() {
        let instruction = INSTRUCTION_MAP.get(&char).unwrap();
        let (dx, dy) = delta_map.get(instruction).unwrap();

        current_node = (current_node.0 + dx, current_node.1 + dy);

        visited_nodes.push(current_node.clone());
    }

    visited_nodes.sort();
    visited_nodes.dedup();

    println!("Total houses visited: {}", visited_nodes.len());
}

fn solve_2() {
    let delta_map = HashMap::from([
        (Instructions::Up, (0, -1)),
        (Instructions::Down, (0, 1)),
        (Instructions::Left, (-1, 0)),
        (Instructions::Right, (1, 0)),
    ]);

    let file_path = "src/day_03/input.txt";
    let input = fs::read_to_string(file_path).expect("could not read input file");
    let input = input.trim();
    let starting_node = (0, 0);
    let mut santa_node = (0, 0);
    let mut robosanta_node = (0, 0);

    let mut visited_nodes: Vec<(isize, isize)> = vec![];

    visited_nodes.push(starting_node.clone());
    visited_nodes.push(starting_node.clone());

    for (idx, char) in input.chars().enumerate() {
        let instruction = INSTRUCTION_MAP.get(&char).unwrap();
        let (dx, dy) = delta_map.get(instruction).unwrap();

        if idx % 2 == 0 {
            santa_node = (santa_node.0 + dx, santa_node.1 + dy);
            visited_nodes.push(santa_node.clone());
        } else {
            robosanta_node = (robosanta_node.0 + dx, robosanta_node.1 + dy);
            visited_nodes.push(robosanta_node.clone());
        }
    }

    visited_nodes.sort();
    visited_nodes.dedup();

    println!(
        "Total houses visited by santa and robosanta: {}",
        visited_nodes.len()
    );
}
