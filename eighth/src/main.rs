mod input;
use crate::input::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct MapInstruction {
    left: String,
    right: String,
}

impl MapInstruction {
    pub fn is_end_node(&self) -> bool {
        if self.left.ends_with('Z') || self.right.ends_with('Z') {
            true
        } else {
            false
        }
    }
}

fn part_one() {
    let mut input_iter = PUZZLE_INPUT.lines();

    // get the step instructions
    let step_instructions: Vec<char> = input_iter.next().unwrap().chars().collect();
    // skip blank line
    let _ = input_iter.next();

    let mut map: HashMap<String, MapInstruction> = HashMap::new();
    // get the step map
    loop {
        if let Some(map_line) = input_iter.next() {
            let (node, instruction) = map_line.split_once(" = ").unwrap();
            let (left_str, right_str) = instruction.split_once(", ").unwrap();

            map.insert(node.to_string(), MapInstruction {
                left: left_str.trim_start_matches('(').to_string(),
                right: right_str.trim_end_matches(')').to_string(),
            });
        } else {
            break;
        }
    }

    let mut step_count = 0;
    let mut found_zzz = false;
    let mut node = "AAA";
    while found_zzz == false {
        for step in &step_instructions {
            if node.contains("ZZZ") {
                found_zzz = true;
                break;   
            }
            let map_instruction = map.get(node).unwrap();
            match step {
                'R' => node = map_instruction.right.as_str(),
                'L' => node = map_instruction.left.as_str(),
                _ => panic!("{step} what?"),
            }
            step_count += 1;
        }
    }

    println!("Value: {}", step_count);
}

fn part_two() {
    
}

fn main() {
    part_one();
    part_two();
}
