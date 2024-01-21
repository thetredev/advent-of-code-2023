use std::collections::HashMap;
use std::fs::read_to_string;

fn cubes_possible(max_cubes: &HashMap<&str, i32>, cube_color_pair: Vec<&str>) -> bool {
    cube_color_pair.first().unwrap().parse::<i32>().unwrap()
        <= max_cubes[cube_color_pair.last().unwrap()]
}

fn is_possible(max_cubes: &HashMap<&str, i32>, game_data: &str) -> bool {
    game_data
        .split("; ") // iterate over cube groups
        .all(|group| {
            group
                .split(", ") // iterate over "<cube count> <cube color>" pairs
                .all(|slice| cubes_possible(max_cubes, slice.split(' ').collect()))
        }) // verify single pair
}

fn game_data_from_line(line: &String) -> &str {
    &line[line.find(':').unwrap() + 2..line.len()]
}

fn main() {
    let filename = std::env::args().nth(1).expect("no input file given");
    let max_cubes: HashMap<&str, i32> = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    let result: usize = read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from) // single line as string
        .enumerate() // game ID = line index + 1
        .filter(|s| is_possible(&max_cubes, game_data_from_line(&s.1))) // filter possible cube groups
        .map(|s| s.0 + 1) // retrieve the game IDs of all matches
        .sum(); // sum of all game IDs

    println!("{}", result);
}
