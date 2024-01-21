use std::collections::HashMap;
use std::fs::read_to_string;

fn cubes_product(game_data: &str) -> i32 {
    let mut cubes: HashMap<&str, i32> = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);

    for mut cube_data in game_data
        .split("; ")
        .flat_map(|group| group.split(", "))
        .map(|raw_data| raw_data.split(' '))
    {
        let cube_color: &str = cube_data.next_back().unwrap();
        let cube_count: i32 = cube_data.next_back().unwrap().parse::<i32>().unwrap();

        // remember maximum value for each color
        if cube_count > cubes[cube_color] {
            cubes.insert(cube_color, cube_count);
        }
    }

    cubes.values().product()
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
