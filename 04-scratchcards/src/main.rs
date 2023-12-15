use std::{env, fs, process};
use regex::{self, Regex};

const MISSING_ARGUMENTS_MESSAGE:&str = "Not enough arguments, please specify an input file";

struct Card {
    winning_numbers: u32,
    copies: u32
}

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

    let mut cards: Vec<Card> = Vec::new();

    for line in lines {
        if line.is_empty() { continue }

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

        let mut numbers: Vec<String> = Vec::new();

        matcher.find_iter(winning_numbers)
               .for_each( |number| numbers.push(number.as_str().to_string()) );

        matcher.find_iter(owned_numbers)
               .for_each ( |number| numbers.push(number.as_str().to_string()) );

        numbers.sort();

        let mut winning_numbers: u32 = 0;

        for i in 0..numbers.len()-1 {
            if numbers[i] == numbers[i+1] {
                winning_numbers += 1
            }
        }

        cards.push( Card { winning_numbers, copies: 1 } )
    }

    for i in 0..cards.len() {
        let card = &cards[i];
        let copies = card.copies;
        let cards_to_copy = card.winning_numbers as usize;

        let mut j: usize = 1;

        while j <= cards_to_copy && j < cards.len() {
            cards[i + j].copies += copies;
            j += 1
        }
    }

    let mut total_scratchcards: u32 = 0;

    for card in cards { total_scratchcards += card.copies };

    println!("Total: {}", total_scratchcards);
}
