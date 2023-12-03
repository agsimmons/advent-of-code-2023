use core::panic;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    part_one();
}

struct Handful {
    num_red: u32,
    num_green: u32,
    num_blue: u32,
}

impl Handful {
    pub fn new(num_red: u32, num_green: u32, num_blue: u32) -> Self {
        Self {
            num_red,
            num_green,
            num_blue,
        }
    }

    pub fn from_handful_string(handful_string: &str) -> Self {
        let mut num_red: u32 = 0;
        let mut num_green: u32 = 0;
        let mut num_blue: u32 = 0;

        let handful_string_parts: Vec<&str> = handful_string.split(",").collect();

        for handful_string_part in handful_string_parts {
            if handful_string_part.contains(" red") {
                num_red = handful_string_part
                    .replace(" red", "")
                    .replace(" ", "")
                    .parse()
                    .expect("Could not parse u32 from part")
            } else if handful_string_part.contains(" green") {
                num_green = handful_string_part
                    .replace(" green", "")
                    .replace(" ", "")
                    .parse()
                    .expect("Could not parse u32 from part")
            } else if handful_string_part.contains(" blue") {
                num_blue = handful_string_part
                    .replace(" blue", "")
                    .replace(" ", "")
                    .parse()
                    .expect("Could not parse u32 from part")
            } else {
                panic!("No color in part");
            }
        }

        Self {
            num_red,
            num_green,
            num_blue,
        }
    }

    pub fn is_possible(&self, limit: &Self) -> bool {
        self.num_red <= limit.num_red
            && self.num_green <= limit.num_green
            && self.num_blue <= limit.num_blue
    }
}

fn part_one() {
    let limit_handful = Handful::new(12, 13, 14);

    let mut possible_game_nums = Vec::<u32>::new();

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(line) = line {
                let line_parts: Vec<&str> = line.split(": ").collect();

                let game_num = line_parts
                    .get(0)
                    .expect("No game string found")
                    .replace("Game ", "")
                    .parse::<u32>()
                    .expect("Could not parse game_num as u32");

                let handfuls: Vec<Handful> = line_parts
                    .get(1)
                    .expect("No handfuls string found")
                    .split("; ")
                    .map(|handful_string| Handful::from_handful_string(handful_string))
                    .collect();

                let game_is_possible = handfuls
                    .iter()
                    .all(|handful| handful.is_possible(&limit_handful));

                if game_is_possible {
                    possible_game_nums.push(game_num);
                }
            }
        }

        println!("Problem 1: {}", possible_game_nums.iter().sum::<u32>())
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
