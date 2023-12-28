use std::fs::read_to_string;

fn main() {
    let filename = std::env::args().nth(1).expect("no input file given");
    let lines: Vec<String> = read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut result: i32 = 0;

    for line in lines.iter() {
        let digits: Vec<char> = line.chars()
            .filter(|c| c.is_digit(10))
            .collect();

        let first: char = *digits.first().unwrap();
        let last: char = *digits.last().unwrap();

        result += format!("{}{}", first, last)
            .parse::<i32>()
            .unwrap();
    }

    println!("{}", result);
}
