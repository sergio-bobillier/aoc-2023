use std::{env, fs, process};
use regex::Regex;

fn read_input(file_name: String) -> String {
    let result = fs::read_to_string(&file_name);

    match result {
        Err(error) => {
            println!("Unable to read input file: {}: {}", file_name, error);
            process::exit(1);
        },
        Ok(text) => return text
    }
}

fn translate(position: usize, width: usize) -> (usize, usize) {
    let x = position % width;
    let y = (position - x) / width;
    (x, y)
}

fn make_box((x, y):(usize, usize), len: usize, width: usize, height: usize) -> (usize, usize, usize, usize) {
    let top: usize = if y < 1 { 0 } else { y - 1 };
    let bottom: usize = if y < height { y + 1 } else { y };
    let left: usize = if x < 1 { 0 } else { x - 1 };
    let right: usize = if x + len > width { width } else { x + len };
    (left, top, right, bottom)
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let file_name = args.pop().unwrap();

    if args.is_empty() {
        panic!("I refuse to read my own binary file as input! Give me a file name!")
    }

    let text: String = read_input(file_name);
    let lines: Vec<&str> = text.split("\n").collect();
    let height = lines.len();
    let width = lines[0].len();

    let stream = text.replace("\n", "");
    let matrix: Vec<Vec<char>> = text.split("\n").map( |line| line.chars().collect() ).collect();

    let matcher = Regex::new("\\d+").unwrap();

    let matches = matcher.find_iter(&stream);

    let mut sum:u32 = 0;
    let symbol_matcher = Regex::new("[^\\d.]").unwrap();

    for matched_item in matches {
        let position = matched_item.start();
        let (x, y) = translate(position, width);
        let (left, top, right, bottom) = make_box((x, y), matched_item.len(), width - 1, height - 1);

        let mut square: String = String::new();

        for i in top..=bottom {
            for j in left..=right {
                square.push(matrix[i][j]);
            }
        }

        if symbol_matcher.is_match(&square) {
            let number = matched_item.as_str().parse::<u32>().unwrap();
            sum += number
        }
    }

    println!("Total is: {}", sum);
}
