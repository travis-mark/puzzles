use aho_corasick::AhoCorasick;
use std::env;
use std::error::Error;
use std::fs;
use std::process;

pub struct Config {
    pub file_path: String,
    pub part2: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let file_path = args[1].clone();
        let part2 = env::var("PART2").is_ok();

        Ok(Config { file_path, part2 })
    }
}

pub fn solve(config: Config) -> Result<u32, Box<dyn Error>> {
    let input = fs::read_to_string(config.file_path)?;
    let patterns: &[&str] = if config.part2 {
        &[
            "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five",
            "six", "seven", "eight", "nine",
        ]
    } else {
        &["1", "2", "3", "4", "5", "6", "7", "8", "9"]
    };
    let ac = AhoCorasick::new(patterns).unwrap();
    let result = input
        .lines()
        .map(|line| {
            let mut matches = ac
                .find_overlapping_iter(line)
                .map(|m| m.pattern().as_usize());

            let first = matches.next().unwrap_or(0) % 9;
            let last = matches.last().unwrap_or(first) % 9;
            let value: u32 = ((first + 1) * 10 + last + 1) as u32;
            println!("{line} {value}");
            return value;
        })
        .sum::<u32>();
    return Ok(result);
}

#[test]
fn sample1() {
    let config = Config {
        file_path: String::from("sample1.txt"),
        part2: false,
    };
    assert_eq!(142, solve(config).unwrap());
}

#[test]
fn sample2() {
    let config = Config {
        file_path: String::from("sample2.txt"),
        part2: true,
    };
    assert_eq!(281, solve(config).unwrap());
}

#[test]
fn input1() {
    let config = Config {
        file_path: String::from("input.txt"),
        part2: false,
    };
    assert_eq!(54697, solve(config).unwrap());
}

#[test]
fn input2() {
    let config = Config {
        file_path: String::from("input.txt"),
        part2: true,
    };
    assert_eq!(54885, solve(config).unwrap());
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
