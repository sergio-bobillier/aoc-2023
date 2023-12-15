use std::fs;

const INPUT_FILE_NAME:&str = "input";

const RED:&str = "red";
const GREEN:&str = "green";
const BLUE:&str = "blue";

const RED_CUBES:u32 = 12;
const GREEN_CUBES:u32 = 13;
const BLUE_CUBES:u32 = 14;


struct Draw {
    red:u32,
    green:u32,
    blue:u32
}

impl Draw {
    pub fn is_valid(&self) -> bool {
        self.red <= RED_CUBES && self.green <= GREEN_CUBES && self.blue <= BLUE_CUBES
    }
}

struct Game {
    id:u32,
    draws:Vec<Draw>
}

impl Game {
    pub fn is_valid(&self) -> bool {
        let mut is_valid:bool = true;

        for draw in &self.draws {
            is_valid = is_valid && draw.is_valid()
        }

        is_valid
    }

    pub fn min_set(&self) -> (u32, u32, u32) {
        let mut max_red:u32 = 0;
        let mut max_green:u32 = 0;
        let mut max_blue:u32 = 0;

        for draw in &self.draws {
            if max_red < draw.red { max_red = draw.red };
            if max_green < draw.green { max_green = draw.green };
            if max_blue < draw.blue { max_blue = draw.blue };
        }

        (max_red, max_green, max_blue)
    }

    pub fn min_power(&self) -> u32 {
        let min_set = self.min_set();
        min_set.0 * min_set.1 * min_set.2
    }
}

fn process_game(id:u32, game_data:&str) -> Game {
    let mut draws:Vec<Draw> = Vec::new();

    let draw_strings = game_data.split(';')
        .map( |draw_string| draw_string.trim() );

    for draw_string in draw_strings {
        let cube_strings = draw_string.split(',')
            .map( |cube_string| cube_string.trim() );

        let mut red:u32 = 0;
        let mut green:u32 = 0;
        let mut blue:u32 = 0;

        for cube_string in cube_strings {
            let mut parts = cube_string.split(' ');

            let number = parts.next().unwrap().parse::<u32>().unwrap();
            let color = parts.next().unwrap();

            match color {
                RED => red += number,
                GREEN => green += number,
                BLUE => blue += number,
                _ => panic!("Unknown color {}", color)
            }
        }

        draws.push(Draw { red, green, blue })
    }

    Game { id, draws }
}

fn validate_games(games:&Vec<Game>) -> u32 {
    let mut sum:u32 = 0;

    for game in games {
        if game.is_valid() {
            sum += game.id;
        }
    }

    sum
}

fn power_sum(games:&Vec<Game>) -> u32 {
    let mut sum:u32 = 0;

    for game in games {
        sum += game.min_power();
    }

    sum
}

fn process(text:String) {
    let lines = text.split('\n');
    let mut games: Vec<Game> = Vec::new();

    for line in lines {
        let mut parts = line.split(':');
        let game_id = parts.next().unwrap();
        let game_data = parts.next().unwrap();

        let parts = game_id.split(' ');
        let id = parts.last().unwrap().parse::<u32>().unwrap();

        games.push(process_game(id, game_data));
    }

    let sum = validate_games(&games);

    println!("The sum of all the valid IDs is: {}", sum);

    let power_sum = power_sum(&games);

    println!("The sum of the powers of the minimum sets is: {}", power_sum);
}

fn main() {
    let result = fs::read_to_string(INPUT_FILE_NAME);

    match result {
        Err(error) => {
            println!("Could not read {}: {}", INPUT_FILE_NAME, error);
        },
        Ok(text) => {
            process(text);
        }
    }
}
