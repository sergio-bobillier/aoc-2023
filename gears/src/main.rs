use std::{env, fs, process, collections::VecDeque};
use regex::Regex;

const NUMBERS:[char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

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

fn is_number(character: char) -> bool {
    NUMBERS.contains(&character)
}

fn expand_number((x, y):(usize, usize), matrix: &Vec<Vec<char>>, width: usize) -> String {
    let mut characters: VecDeque<char> = VecDeque::new();

    characters.push_front(matrix[y][x]);

    let mut i: usize = x;

    while i > 0 {
        i -= 1;

        let character = matrix[y][i];
        if is_number(character) {
            characters.push_front(character)
        } else {
            break;
        }
    }

    i = x + 1;

    while i < width {
        let character = matrix[y][i];
        if is_number(character) {
            characters.push_back(character)
        } else {
            break;
        }
        i += 1;
    }

    String::from_iter(characters)
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

    let star_matcher = Regex::new("\\*").unwrap();

    let matches = star_matcher.find_iter(&stream);

    let mut sum:u32 = 0;

    for matched_item in matches {
        let position = matched_item.start();
        let (x, y) = translate(position, width);
        let (left, top, right, bottom) = make_box((x, y), matched_item.len(), width - 1, height - 1);

        let mut numbers_count = 0;
        let mut numbers_cords: Vec<(usize, usize)> = Vec::new();
        let mut in_number = false;

        for i in top..=bottom {
            for j in left..=right {
                let character = matrix[i][j];

                if is_number(character) {
                    if !in_number {
                        numbers_cords.push((j, i));
                        numbers_count += 1;
                        in_number = true;
                    }
                } else {
                    in_number = false;
                }
            }

            in_number = false;
        }

        if numbers_count == 2 {
            let mut ratio: u32 = 1;

            for cord in numbers_cords {
                let number_string = expand_number(cord, &matrix, width);
                let number = number_string.parse::<u32>().unwrap();
                ratio *= number;
            }

            sum += ratio;
        }
    }

    println!("Total is: {}", sum);
}
