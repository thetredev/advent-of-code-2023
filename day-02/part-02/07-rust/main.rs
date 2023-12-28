use std::collections::HashMap;
use std::fs::read_to_string;

fn cubes_product(game_data: &str) -> i32 {
    let mut cube_data: HashMap<&str, i32> = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);

    game_data
        .split("; ")
        .collect::<Vec<&str>>()
        .iter() // iterate over cube groups
        .for_each(|group| {
            group
                .split(", ")
                .collect::<Vec<&str>>()
                .iter() // iterate over "<cube count> <cube color>" pairs
                .for_each(|slice| {
                    let cube_slice: Vec<&str> = slice.split(' ').collect::<Vec<&str>>();
                    let cube_count: i32 = cube_slice.first().unwrap().parse::<i32>().unwrap();
                    let cube_color: &str = cube_slice.last().unwrap();

                    // remember maximum value for each color
                    if cube_count > cube_data[cube_color] {
                        cube_data.insert(cube_color, cube_count);
                    }
                });
        });

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
