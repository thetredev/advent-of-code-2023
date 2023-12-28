use std::collections::HashMap;
use std::fs::read_to_string;

fn is_possible(max_cubes: &HashMap<&str, i32>, data: &str) -> bool {
    let groups: Vec<&str> = data.split("; ").collect();

    for group in groups.iter() {
        let slices: Vec<&str> = group.split(", ").collect();

        for slice in slices.iter() {
            let value: Vec<&str> = slice.split(" ").collect();

            let cubes: i32 = value
                .first()
                .unwrap()
                .parse::<i32>()
                .unwrap();

            let mut color: String = value.last().unwrap().to_string();

            if color.ends_with("\n") {
                color.pop();
            }

            if cubes > max_cubes[&*color] {
                return false;
            }
        }
    }

    return true;
}

fn main() {
    let filename = std::env::args().nth(1).expect("no input file given");
    let lines: Vec<String> = read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let max_cubes: HashMap<&str, i32> = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14)
    ]);

    let mut result: i32 = 0;
    let mut game_id: i32 = 0;

    for line in lines.iter() {
        game_id += 1;

        let data_start: usize = line.find(":").unwrap() + 2;
        let data: &str = &line[data_start..line.len()];

        if is_possible(&max_cubes, data) {
            result += game_id;
        }
    }

    println!("{}", result);
}
