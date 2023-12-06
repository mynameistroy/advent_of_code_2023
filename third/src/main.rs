mod input;
use core::num;

use crate::input::{EXAMPLE_INPUT, PUZZLE_INPUT, ANOTHER_EXAMPLE};

fn part_one() {
    let mut schematic: Vec<Vec<char>> = Vec::new();

    for line in EXAMPLE_INPUT.lines() {
        schematic.push(line.chars().collect());
    }

    let mut part_numbers: Vec<usize> = Vec::new();
    let mut line_number = 0;
    for line in &schematic {
        let mut part_number = String::default();
        let mut point_number = 0;
        let mut is_part = false;
        for point in line {
            //println!("point_number {point_number} line_number {line_number}");
            match point {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    part_number.push(*point);

                    let start_check_point = if point_number == 0 {
                        point_number
                    } else {
                        point_number - 1
                    };

                    let end_check_point = if point_number > (line.len() - 1) {
                        point_number
                    } else {
                        point_number + 1
                    };

                    if point_number > 0 && point_number < line.len() - 1 {
                        if line[start_check_point] != '.' && !line[start_check_point].is_ascii_digit() {
                            is_part = true;
                        } else if line[end_check_point] != '.' && !line[end_check_point].is_ascii_digit() {
                            is_part = true;
                        }
                    }
                    
                    if line_number > 0 && is_part == false {
                        let above_line = &schematic[line_number - 1];
                        if let Some(start) = above_line.get(start_check_point..end_check_point+1) {
                            //println!("above {point} {start_check_point} {end_check_point} {:?}", start);
                            for above_point in start {
                                if *above_point != '.' && !above_point.is_ascii_digit() {
                                    is_part = true;
                                    //print!("{above_point}:{is_part} ");
                                }
                            }
                        }
                        //println!("\n");
                    }

                    if line_number < schematic.len() - 1 && is_part == false {
                        let below_line = &schematic[line_number + 1];
                        if let Some(start) = below_line.get(start_check_point..end_check_point+1) {
                            //println!("below {point} {start_check_point} {end_check_point} {:?}", start);
                            for below_point in start {
                                if *below_point != '.' && !below_point.is_ascii_digit() {
                                    is_part = true;
                                }
                                //print!("{below_point}:{is_part} ");
                            }
                            //println!("\n");
                        }
                    }
                },
                _ => {
                    if !part_number.is_empty() {
                        let part: usize = part_number.parse().unwrap();

                        //println!("{part} {is_part}");
                        // if it preceded or succeeded the number on the same line
                        if is_part {
                            for c in part_number.chars() {
                                print!("{c}");
                            }
                            part_numbers.push(part);
                        } else {
                            print!("{part}");
                        }
                        is_part = false;
                        part_number.clear();
                    } 
                    print!("{point}");
                },
            }
            point_number += 1;
        }
        if !part_number.is_empty() {
            let part: usize = part_number.parse().unwrap();
            
            // if it preceded or succeeded the number on the same line
            if is_part {
                for c in part_number.chars() {
                    print!("{c}");
                }
                part_numbers.push(part);
            } else {
                print!("{part}");
            }
            part_number.clear();
        }
        println!("");
        line_number += 1;
    }
    let part_numbers_sum: usize = part_numbers.iter().sum();
    println!("Value: {part_numbers_sum}");
}

fn check_line_left(line: &[char], point: usize) -> Option<usize> {
    let mut number = String::default();
    for c in line[..point].into_iter().rev() {
        if c.is_ascii_digit() {
            number.insert(0, *c);
        } else {
            break;
        }
    }
    if !number.is_empty() {
        return Some(number.parse().unwrap());
    } else {
        None
    }
}

fn check_line_right(line: &[char], point: usize) -> Option<usize> {
    let mut number = String::default();
    for c in line[point + 1..].into_iter() {
        if c.is_ascii_digit() {
            number.push(*c);
        } else {
            break;
        }
    }
    if !number.is_empty() {
        return Some(number.parse().unwrap());
    } else {
        None
    }
}

fn check_directly(line: &[char], point: usize) -> Option<usize> {
    let mut number = String::from(line[point]);
    for c in line[..point].into_iter().rev() {
        if c.is_ascii_digit() {
            number.insert(0, *c);
        } else {
            break;
        }
    }
    for c in line[point + 1..].into_iter() {
        if c.is_ascii_digit() {
            number.push(*c);
        } else {
            break;
        }
    }
    if !number.is_empty() {
        return Some(number.parse().unwrap());
    } else {
        None
    }
}

fn part_two() {
    let mut schematic: Vec<Vec<char>> = Vec::new();

    for line in PUZZLE_INPUT.lines() {
        schematic.push(line.chars().collect());
    }

    let mut gears: Vec<usize> = Vec::new();
    let mut line_number = 0;
    for line in &schematic {
        let mut point_number = 0;
        for point in line {
            // is it a gear?
            if *point == '*' {
                let mut part_numbers: Vec<usize> = Vec::new();

                // check above
                if line_number > 0 {
                    let line_above = &schematic[line_number - 1];
                    // is the space above a number
                    if !line_above[point_number].is_ascii_digit() {
                        if point_number > 0 && line_above[point_number - 1].is_ascii_digit() {
                            // there is a number to left
                            if let Some(part) = check_line_left(&line_above, point_number) {
                                part_numbers.push(part);
                            }
                        }
                        if point_number < line.len() - 1 && line_above[point_number + 1].is_ascii_digit() {
                            // there is a number to right
                            if let Some(part) = check_line_right(&line_above, point_number) {
                                part_numbers.push(part);
                            }
                        }
                    } else {
                        if let Some(part) = check_directly(&line_above, point_number) {
                            part_numbers.push(part);
                        }
                    }
                }

                // check below
                if line_number != schematic.len() - 1 {
                    let line_below = &schematic[line_number + 1];
                    // is the space above a number
                    if !line_below[point_number].is_ascii_digit() {
                        if point_number > 0 && line_below[point_number - 1].is_ascii_digit() {
                            // there is a number to left
                            if let Some(part) = check_line_left(&line_below, point_number) {
                                part_numbers.push(part);
                            }
                        }
                        if point_number < line.len() - 1 && line_below[point_number + 1].is_ascii_digit() {
                            // there is a number to right
                            if let Some(part) = check_line_right(&line_below, point_number) {
                                part_numbers.push(part);
                            }
                        }
                    } else {
                        if let Some(part) = check_directly(&line_below, point_number) {
                            part_numbers.push(part);
                        }
                    }
                }

                // check left side
                if point_number > 0 {
                    if let Some(part) = check_line_left(&line, point_number) {
                        part_numbers.push(part);
                    }
                }

                // check right side
                if point_number < line.len() - 1 {
                    if let Some(part) = check_line_right(&line, point_number) {
                        part_numbers.push(part);
                    }
                }

                if part_numbers.len() == 2 {
                    // it's a real gear, get the ratio
                    gears.push(part_numbers[0] * part_numbers[1]);
                }
            }
            point_number += 1;
        }
        line_number += 1;
    }
    println!("Value: {}", gears.iter().sum::<usize>());
}

fn main() {
    part_one();
    part_two();
}
