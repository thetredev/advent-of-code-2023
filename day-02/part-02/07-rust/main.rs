use std::collections::HashMap;
use std::fs::read_to_string;

fn cubes_product(game_data: &str) -> i32 {
    let mut cube_data: HashMap<&str, i32> = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);

    for group in game_data.split("; ") {
        for slice in group.split(", ") {
            let mut cube_slice = slice.split(' ');
            let cube_color: &str = cube_slice.next_back().unwrap();
            let cube_count: i32 = cube_slice.next_back().unwrap().parse::<i32>().unwrap();

            // remember maximum value for each color
            if cube_count > cube_data[cube_color] {
                cube_data.insert(cube_color, cube_count);
            }
        }
    }

    cube_data.values().product()
}

fn game_data_from_line(line: &String) -> &str {
    &line[line.find(':').unwrap() + 2..line.len()]
}

fn main() {
    let filename = std::env::args().nth(1).expect("no input file given");

    let result: i32 = read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from) // single line as string
        .map(|s| cubes_product(game_data_from_line(&s)))
        .sum();

    println!("{}", result);
}
