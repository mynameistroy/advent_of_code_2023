mod input;
use input::*;

fn part_one() {
    let mut lines_iter = PUZZLE_INPUT.lines();
    let times_str = lines_iter.next().unwrap();
    let distances_str = lines_iter.next().unwrap();
    let (_, times_str) = times_str.split_once(":").unwrap();
    let times: Vec<usize> = times_str.split_whitespace().into_iter().map(|x| x.trim().parse().unwrap() ).collect();
    let (_, distances_str) = distances_str.split_once(":").unwrap();
    let distances: Vec<usize> = distances_str.split_whitespace().into_iter().map(|x| x.trim().parse().unwrap() ).collect();

    let mut value = 1;
    for i in 0..times.len() {
        let race_time = times[i];
        let race_distance = distances[i];
        let mut winning_races = 0;
        for pressed in 0..race_time {
            let race_time_left = race_time - pressed;
            let distance_traveled = race_time_left * pressed;
            if distance_traveled > race_distance {
                winning_races += 1;
            }
        }
        value *= winning_races;
    }

    println!("Value:{}", value);
}

fn part_two() {
    let mut lines_iter = PUZZLE_INPUT.lines();
    let times_str = lines_iter.next().unwrap();
    let distances_str = lines_iter.next().unwrap();
    let (_, times_str) = times_str.split_once(":").unwrap();
    let times: Vec<&str> = times_str.split_whitespace().into_iter().collect();
    let race_time: usize = times.join("").parse().unwrap();
    let (_, distances_str) = distances_str.split_once(":").unwrap();
    let distances: Vec<&str> = distances_str.split_whitespace().into_iter().collect();
    let race_distance: usize = distances.join("").parse().unwrap();

    let mut winning_races = 0;
    for pressed in 0..race_time {
        let race_time_left = race_time - pressed;
        let distance_traveled = race_time_left * pressed;
        if distance_traveled > race_distance {
            winning_races += 1;
        }
    }

    println!("Value:{}", winning_races);
}

fn main() {
    part_one();
    part_two();
}
