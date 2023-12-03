use std::env;
use std::error::Error;
use std::fs;
use std::process;

struct Config {
    pub file_path: String,
    pub part2: bool,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let file_path = args[1].clone();
        let part2 = env::var("PART2").is_ok();

        Ok(Config { file_path, part2 })
    }
}

struct Game {
    id: u32,
    red: u32, green: u32, blue: u32
}

impl Game {
    fn new(line: &str) -> Game {
        let mut red: u32 = 0;
        let mut green: u32 = 0;
        let mut blue: u32 = 0;

        // TODO: Play with different parsers
        let parts: Vec<&str> = line.split(": ").collect();
        let id = parts[0].split(" ").collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
        let samples: Vec<&str> = parts[1].split("; ").collect();
        for sample in samples {
            let data: Vec<&str> = sample.split(", ").collect();
            for datum in data {
                let pair: Vec<&str> = datum.split(" ").collect();
                let value = pair[0].parse::<u32>().unwrap();
                match pair[1] {
                    "red" => red = if value > red { value } else { red },
                    "green" => green = if value > green { value } else { green },
                    "blue" => blue = if value > blue { value } else { blue },
                    _ => continue
                }
             }
        }

        return Game { id: id, red: red, green: green, blue: blue};
    }
    
    fn is_legal(game: &Game) -> bool {
        return game.red <= 12 && game.green <= 13 && game.blue <= 14;
    }
}

fn solve(config: Config) -> Result<u32, Box<dyn Error>> {
    let input: String = fs::read_to_string(config.file_path)?;
    let games = input
        .lines()
        .map(|line| { return Game::new(line); });
    let answer: u32 = if config.part2 {
        games
            .map(|game| game.red * game.blue * game.green)
            .sum()
    } else {
        games
            .filter(|game| { return Game::is_legal(game); })
            .map(|game| game.id)
            .sum()
    };
    return Ok(answer);
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("In file {}", config.file_path);
    let answer = solve(config).unwrap_or_else(|err| {
        eprintln!("Application error: {err}");
        process::exit(1);
    });
    println!("Answer: {answer}");
}

#[test]
fn sample1() {
    let config = Config {
        file_path: String::from("sample.txt"),
        part2: false,
    };
    assert_eq!(8, solve(config).unwrap());
}

#[test]
fn sample2() {
    let config = Config {
        file_path: String::from("sample.txt"),
        part2: true,
    };
    assert_eq!(2286, solve(config).unwrap());
}

#[test]
fn part1() {
    let config = Config {
        file_path: String::from("input.txt"),
        part2: false,
    };
    assert_eq!(2795, solve(config).unwrap());
}

#[test]
fn part2() {
    let config = Config {
        file_path: String::from("input.txt"),
        part2: true,
    };
    assert_eq!(75561, solve(config).unwrap());
}