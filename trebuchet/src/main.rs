use std::fs;
use regex::Regex;

const FILENAME:&str = "input";

fn parse_text(text: String) -> u32 {
    let lines = text.split('\n');
    let mut total: u32 = 0;

    let regexp = Regex::new(r"\d").unwrap();

    for line in lines {
        let matches:Vec<_> = regexp.find_iter(line)
                                   .map(|m| m.as_str()).collect();


        let first_match = matches.first();
        let last_match = matches.last();

        if first_match.is_none() || last_match.is_none() {
            panic!("Could not find digits in line '{}'", line);
        }

        let first_digit = first_match.unwrap().to_string();
        let last_digit = last_match.unwrap().to_string();

        let mut line_digit_str = first_digit.clone();
        line_digit_str.push_str(&last_digit);

        let parse_result = line_digit_str.parse::<u32>();

        match parse_result {
            Err(error) => {
                panic!(
                    "Unable to parse '{}' from line '{}': {}",
                    line_digit_str, line, error
                );
            }
            Ok(line_digit) => {
                total += line_digit
            }
        }
    }

    total
}

fn main() {
    println!("Reading {}...", FILENAME);

    let result = fs::read_to_string(FILENAME);

    match result {
        Err(error) => {
            println!("Failed to load file '{}': {}", FILENAME, error)
        }
        Ok(text) => {
            let calibration_number = parse_text(text);
            println!("Total calibration number is: {}", calibration_number)
        }
    }
}
