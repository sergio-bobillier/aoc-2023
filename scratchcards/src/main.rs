use std::{env, fs, process};
use regex::{self, Regex};

const MISSING_ARGUMENTS_MESSAGE:&str = "Not enough arguments, please specify an input file";

fn read_input(file_name: String) -> String {
    let result = fs::read_to_string(&file_name);

    match result {
        Err(error) => {
            println!("Unable to read file '{}': {}", file_name, error);
            process::exit(1);
        },
        Ok(text) => text
    }
}

fn main() {
    let mut args: Vec<String> = env::args().collect();

    let file_name: String = args.pop().expect(MISSING_ARGUMENTS_MESSAGE);

    if args.is_empty() {
        panic!("{}", MISSING_ARGUMENTS_MESSAGE)
    }

    let text = read_input(file_name);
    let lines = text.split('\n');

    let mut total_points:u32 = 0;

    for line in lines {
        if line.is_empty() { continue }

        let mut card_points: u32 = 0;

        let mut parts = line.split('|');

        let left_part = parts.next().expect(
            &format!("Wrong input format at line '{}': No vertical bar found", line)
        );

        let owned_numbers = parts.next().expect(
            &format!("Wrong input format at line '{}': Nothing found after the vertical bar", line)
        );

        parts = left_part.split(':');

        let winning_numbers = parts.last().expect(
            &format!("Wrong input format at line '{}': No winning numbers found!", line)
        );

        let matcher = Regex::new("\\d+").expect("Regex compilation failed!");

        let mut numbers: Vec<&str> = Vec::new();

        matcher.find_iter(winning_numbers)
               .for_each( |number| numbers.push(number.as_str()) );

        matcher.find_iter(owned_numbers)
               .for_each ( |number| numbers.push(number.as_str()) );

        numbers.sort();

        for i in 0..numbers.len()-1 {
            if numbers[i] == numbers[i+1] {
                if card_points == 0 {
                    card_points = 1
                } else {
                    card_points *= 2;
                }
            }
        }

        total_points += card_points;
    }

    println!("Total: {}", total_points);
}
