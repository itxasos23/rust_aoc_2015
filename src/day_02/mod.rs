use std::fs;

pub fn solve() {
    println!("Solving Day 2 - Part 1");
    solve_1();

    println!("Solving Day 2 - Part 2");
    solve_2();
}

fn get_box_dimensions() -> Vec<Vec<usize>> {
    let file_path = "src/day_02/input.txt";
    let input = fs::read_to_string(file_path).expect("Can't read input file.");

    let boxes_strings = input.split("\n");
    let mut boxes_dimensions = vec![];
    for box_string in boxes_strings {
        if box_string == "" {
            break;
        }

        let parts = box_string.split("x");
        let mut box_dimensions = vec![];
        for part in parts {
            box_dimensions.push(part.parse::<usize>().expect("Can't parse box dimension."));
        }
        boxes_dimensions.push(box_dimensions);
    }

    boxes_dimensions
}

fn solve_1() {
    let boxes_dimensions = get_box_dimensions();

    let mut sum = 0;

    for box_dimensions in boxes_dimensions {
        let [x, y, z] = box_dimensions.as_slice() else {panic!("Bad format for box_dimensions")};
        let sides: Vec<usize> = vec![x*y, x*z, y*z];
        let extra_side = sides
            .iter()
            .min()
            .expect("Can't calculate minimum of box_dimensions.");
        let total_paper = 2 * sides.iter().sum::<usize>() + extra_side;
        // println!("Total paper needed: {}", total_paper);
        sum += total_paper;
    }

    println!("Total paper needed: {}", sum);
}

fn solve_2() {
    let boxes_dimensions = get_box_dimensions();

    let mut sum = 0;
    for box_dimensions in boxes_dimensions {
        let [x, y, z] = box_dimensions.as_slice() else {panic!("Bad format for box_dimensions")};
        let perimeters: Vec<usize> = vec![2*x+2*y, 2*x+2*z, 2*y+2*z];
        let ribbon = perimeters 
            .iter()
            .min()
            .expect("Can't calculate minimum of box_dimensions.");
        let volume = x * y * z; 
        let total_ribbon = ribbon + volume; 
        // println!("Total paper needed: {}", total_paper);
        sum += total_ribbon;
    }

    println!("Total ribbon needed: {}", sum);
}
