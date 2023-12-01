use std::fs;
use regex::Regex;

const FILENAME:&str = "input";
const NUMBERS:[&str; 10] = ["\\d", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn translate(number: &str) -> u32 {
    let index_option = NUMBERS.iter().position(|candidate| candidate == &number);

    match index_option {
        Some(index) => {
            return index as u32;
        }
        None => {
            let parse_result = number.parse::<u32>();

            match parse_result {
                Err(error) => {
                    panic!(
                        "Unable to parse '{}'': {}",
                        number, error
                    );
                }
                Ok(number) => {
                    return number;
                }
            }
        }
    }
}

fn parse_text(text: String) -> u32 {
    let lines = text.split('\n');
    let mut total: u32 = 0;

    let x = NUMBERS.join("|");
    let regexp = Regex::new(&x).unwrap();

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

        let first_number= translate(&first_digit);
        let last_number = translate(&last_digit);

        let number = (first_number * 10 + last_number);
        total += number;

        println!("{} -> {}:{} -> {}:{} -> {} -> {}", line, first_digit, last_digit, first_number, last_number, number, total)
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
