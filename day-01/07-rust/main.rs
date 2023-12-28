use std::fs::read_to_string;

fn chars_to_int(chars: &Vec<char>) -> i32 {
    let first: char = *chars.first().unwrap();
    let last: char = *chars.last().unwrap();

    format!("{}{}", first, last)
        .parse::<i32>()
        .unwrap()
}

fn main() {
    let filename = std::env::args().nth(1).expect("no input file given");
    let lines: Vec<String> = read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let result: i32 = lines.iter()
        .map(|line| chars_to_int(&line.chars()
            .filter(|c| c.is_digit(10))
            .collect::<Vec<char>>()))
        .sum();

    println!("{}", result);
}
