use crate::input::PUZZLE_INPUT;
mod input;

#[derive(Debug)]
struct Bag {
    red: usize,
    blue: usize,
    green: usize,
}

const MAX_RED: usize = 12;
const MAX_GREEN: usize = 13;
const MAX_BLUE: usize = 14;

fn part_one() {
    let mut possible_games: Vec<usize> = Vec::new();

    for line in PUZZLE_INPUT.lines() {
        if line.is_empty() {
            continue;
        }

        let (game_number, games_played) = line.split_once(':').unwrap();
        let(_, game_id) = game_number.split_once(' ').unwrap();
        let mut is_possible = true;
        for game in games_played.split(';') {
            let mut bag = Bag {
                red: 0,
                blue: 0,
                green: 0,
            };
            for cubes in game.trim().split(',') {
                let whitespace = cubes.trim().find(char::is_whitespace).unwrap();
                let (count, colour) = cubes.trim().split_at(whitespace);
                match colour.trim() {
                    "blue" => bag.blue = count.trim().parse::<usize>().unwrap(),
                    "red" => bag.red = count.trim().parse::<usize>().unwrap(),
                    "green" => bag.green = count.trim().parse::<usize>().unwrap(),
                    _ => panic!("huh? count:{count} colour:{colour}"),
                }
            }
            if bag.red > MAX_RED || bag.green > MAX_GREEN || bag.blue > MAX_BLUE {
                is_possible = false;
            }
        }
        if is_possible {
            possible_games.push(game_id.parse().unwrap());
        }
    }
    let sum: usize = possible_games.iter().sum();
    println!("Value: {sum}");
}

fn part_two() {
    let mut cube_powers: Vec<usize> = Vec::new();

    for line in PUZZLE_INPUT.lines() {
        if line.is_empty() {
            continue;
        }

        let (_, games_played) = line.split_once(':').unwrap();
        let mut bag = Bag {
            red: 0,
            blue: 0,
            green: 0,
        };
        for game in games_played.split(';') {
            for cubes in game.trim().split(',') {
                let whitespace = cubes.trim().find(char::is_whitespace).unwrap();
                let (count, colour) = cubes.trim().split_at(whitespace);
                match colour.trim() {
                    "blue" => bag.blue = {
                        let numeric_count = count.trim().parse::<usize>().unwrap();
                        if numeric_count > bag.blue {
                            numeric_count
                        } else {
                            bag.blue
                        }
                    },
                    "red" => bag.red = {
                        let numeric_count = count.trim().parse::<usize>().unwrap();
                        if numeric_count > bag.red {
                            numeric_count
                        } else {
                            bag.red
                        }
                    },
                    "green" => bag.green = {
                        let numeric_count = count.trim().parse::<usize>().unwrap();
                        if numeric_count > bag.green {
                            numeric_count
                        } else {
                            bag.green
                        }
                    },
                    _ => panic!("huh? count:{count} colour:{colour}"),
                }
            }
        }
        let cube_power = bag.red * bag.blue * bag.green;
        cube_powers.push(cube_power);
    }
    let sum: usize = cube_powers.iter().sum();
    println!("Value: {sum}");
}

fn main() {
    part_one();
    part_two();
}
