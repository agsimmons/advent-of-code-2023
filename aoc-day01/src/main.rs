use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// static digit_strings: u32 = 4;

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut nums = Vec::<u32>::new();

        for line in lines {
            if let Ok(line) = line {
                let digits: Vec<u32> = line
                    .chars()
                    .map(|char| char.to_digit(10))
                    .flatten()
                    .collect();

                let first_digit_string = digits.first().unwrap().to_string();
                let last_digit_string = digits.last().unwrap().to_string();
                if let Ok(num) = format!("{first_digit_string}{last_digit_string}").parse::<u32>() {
                    nums.push(num);
                }
            }
        }

        println!("Puzzle 1: {}", nums.iter().sum::<u32>());
    }
}

fn get_first_num(line: &str, reverse: bool) -> Option<u32> {
    let digit_strings = HashMap::<&str, u32>::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let mut char_buffer = String::new();

    if reverse {
        for char in line.chars().rev() {
            if let Some(digit) = char.to_digit(10) {
                return Some(digit);
            } else {
                char_buffer.insert(0, char);

                for &digit_string in digit_strings.keys() {
                    if char_buffer.contains(digit_string) {
                        if let Some(digit_string_value) = digit_strings.get(digit_string) {
                            return Some(digit_string_value.clone());
                        }
                    }
                }
            }
        }
    } else {
        for char in line.chars() {
            if let Some(digit) = char.to_digit(10) {
                return Some(digit);
            } else {
                char_buffer.push(char);

                for &digit_string in digit_strings.keys() {
                    if char_buffer.contains(digit_string) {
                        if let Some(digit_string_value) = digit_strings.get(digit_string) {
                            return Some(digit_string_value.clone());
                        }
                    }
                }
            }
        }
    }

    return None;
}

fn part_two() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut nums = Vec::<u32>::new();

        for line in lines {
            if let Ok(line) = line {
                let first_num = get_first_num(line.as_str(), false).expect("first_num not found");
                let last_num = get_first_num(line.as_str(), true).expect("first_num not found");
                // let last_num = get_first_num(line.chars().rev().collect::<String>().as_str())
                // .expect("last_num not found");

                if let Ok(num) = format!("{first_num}{last_num}").parse::<u32>() {
                    nums.push(num);
                }
            }
        }

        println!("Puzzle 2: {}", nums.iter().sum::<u32>());
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
