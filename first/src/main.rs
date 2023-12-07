mod input;
use crate::input::PUZZLE_INPUT;

fn part_one() {
    let lines = PUZZLE_INPUT.split_terminator('\n');
    let mut total = 0;
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let mut value = String::default();
        for c in line.chars() {
            if c.is_numeric() {
                value.push(c); 
                break;               
            }
        }
        for c in line.chars().rev() {
            if c.is_numeric() {
                value.push(c); 
                break;               
            }
        }
        let numeric_value: u32 = value.trim().parse().unwrap();
        total += numeric_value;
    }
    println!("Value:{total}");
}

pub fn part_two() {
    let numbers: Vec<&'static str> = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let lines = PUZZLE_INPUT.split_terminator('\n');
    let mut total = 0;
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let mut value = String::default();
        for (index, c) in line.char_indices() {
            if c.is_numeric() {
                value.push(c); 
                break;               
            } else if let Some(string) = line.get(index..line.len()) {
                let mut idx = 0;
                for number in &numbers {
                    if string.starts_with(number) {
                        value.push(char::from_digit(idx + 1, 10).unwrap());
                        break;
                    }
                    idx += 1;
                }
                if idx < numbers.len() as u32 {
                    break;
                }
            }
        }
        for (index, c) in line.char_indices().rev() {
            if c.is_numeric() {
                value.push(c); 
                break;               
            } else if let Some(string) = line.get(index..line.len()) {
                let mut idx = 0;
                for number in &numbers {
                    if string.starts_with(number) {
                        value.push(char::from_digit(idx + 1, 10).unwrap());
                        break;
                    }
                    idx += 1;
                }
                if idx < numbers.len() as u32 {
                    break;
                }
            }
        }
        let numeric_value: u32 = value.trim().parse().unwrap();
        total += numeric_value;
    }
    println!("Value:{total}");
}

pub fn main() {
    part_one();
    part_two();
}