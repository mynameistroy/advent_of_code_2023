mod input;
use crate::input::{EXAMPLE_INPUT, PUZZLE_INPUT, ANOTHER_EXAMPLE};

fn part_one() {
    let mut schematic: Vec<Vec<char>> = Vec::new();

    for line in PUZZLE_INPUT.lines() {
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
                            for _c in part_number.chars() {
                                print!("_");
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
                for _c in part_number.chars() {
                    print!("_");
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

fn part_two() {

}

fn main() {
    part_one();
    part_two();
}
