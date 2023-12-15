use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp;

#[derive(Debug)]
struct GameRound {
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(Debug)]
struct Game {
    id: i32,
    rounds: Vec<GameRound>
}

pub fn run() {
    let games = load_games();
    let condition = GameRound {
        red: 12u8,
        green: 13u8,
        blue: 14u8
    };

    let mut possibles_sum = 0;
    let mut power_sum = 0;
    for game in games {
        if is_game_possible(&game, &condition) {
            possibles_sum += game.id;
        }
        power_sum += get_power(&game);
    }
    println!("possibles sum: {}", possibles_sum); // 2416 = part 1
    println!("power sum: {}", power_sum);
}

fn is_game_possible(game: &Game, condition: &GameRound) -> bool {
    let mut possible = true;
    for round in &game.rounds {
        possible = possible && is_round_possible(&round, condition);
    }
    return possible;
}

fn is_round_possible(round: &GameRound, condition: &GameRound) -> bool {
    return condition.red >= round.red
        && condition.green >= round.green
        && condition.blue >= round.blue
}

// Part 2
fn get_power(game: &Game) -> i32 {
    let mut min_red = 0;
    let mut min_green = 0;
    let mut min_blue = 0;
    for round in &game.rounds {
        min_red = cmp::max(round.red, min_red);
        min_green = cmp::max(round.green, min_green);
        min_blue = cmp::max(round.blue, min_blue);
    }

    return i32::from(min_red)
        * i32::from(min_green)
        * i32::from(min_blue);
}

fn load_games() -> Vec<Game> {
    let mut games: Vec<Game> = Vec::new();
    // let file = "./inputs/day2_sample.txt";
    let file = "./inputs/day2.txt";
    if let Ok(lines) = read_lines(file) {
        for line in lines {
            if let Ok(game_line) = line {
                let id_to_results = game_line.split_once(":").unwrap();
                let game_id = id_to_results.0.split_once(" ").unwrap().1;
                games.push(Game { 
                    id: game_id.parse::<i32>().unwrap(),
                    rounds: read_results(id_to_results.1)
                });
            }
        }
    }
    return games;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_results(result_string: &str) -> Vec<GameRound> {
    let mut rounds: Vec<GameRound> = Vec::new();
    for result in result_string.split(";") {
        let mut red = 0u8;
        let mut green = 0u8;
        let mut blue = 0u8;
        for attributes in result.split(",") {
            let attribute_split = attributes.trim().split_once(" ");
            if let Some((count, color)) = attribute_split {
                let value = count.parse::<u8>().unwrap();
                match color {
                    "red" => red = value,
                    "green" => green = value,
                    "blue" => blue = value,
                    _ => {}
                }

            }
        }
        let round = GameRound { red, green, blue };
        rounds.push(round);
    }
    return rounds;
}
