mod input;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::sync::Arc;

use crate::input::{EXAMPLE_INPUT, PUZZLE_INPUT};

#[derive(Debug, Clone)]
struct Table {
    offset_lookup: Vec<RemapData>,
}

#[derive(Debug, Clone)]
struct RemapData {
    pub dest: usize,
    pub src: usize,
    pub range: usize,
}

impl Table {
    pub fn new() -> Self {
        Table {
            offset_lookup: Vec::new(),
        }
    }

    pub fn add_table(&mut self, table: &str) {
        let fields: Vec<usize> = table.split_whitespace().into_iter().map(|x| x.parse().unwrap()).collect();
        self.offset_lookup.push( RemapData {
            dest: fields[0],
            src: fields[1],
            range: fields[2],
        });
    }

    pub fn lookup(&self, lookup: usize) -> usize {
        for remap_data in &self.offset_lookup {
            if lookup >= remap_data.src && lookup < remap_data.src + remap_data.range {
                return (lookup - remap_data.src) + remap_data.dest;
            }
        }
        lookup
    }
}

fn part_one() {
    let mut seed_data: String = String::default();
    let mut lookup_tables: Vec<Table> = Vec::new();

    let mut input_iter = PUZZLE_INPUT.lines();
    let mut not_done = true;
    while not_done {
        let line = if let Some(line) = input_iter.next() {
            line
        } else {
            not_done = false;
            continue;
        };
        
        if line.contains("seeds: ") {
            println!("getting seeds...");
            let (_, seed_str) = line.split_once(": ").unwrap();
            seed_data = seed_str.to_string();
        }

        if line.contains("map") {
            print!("building {}", line);
            let mut map_not_done = true;
            let mut map_table = Table::new();
            while map_not_done {
                if let Some(map_data) = input_iter.next() {
                    if map_data.is_empty() {
                        map_not_done = false;
                        continue;
                    } 
                    map_table.add_table(map_data);
                } else {
                    not_done = false;
                    break;
                }
            }
            println!("done");
            lookup_tables.push(map_table);
        }
    }

    println!("mapping seed to loc...");
    let lowest_loc = Arc::new(AtomicUsize::new(usize::MAX));
    let mut threads = Vec::new();
    let arc_lookup_tables = Arc::new(lookup_tables);
    for seed in seed_data.split_whitespace().into_iter().map(|x| x.parse().unwrap()).collect::<Vec<usize>>() {
        let mut value = seed.clone();
        let thread_lookup_tables = arc_lookup_tables.clone();
        let lowest_loc_ref = lowest_loc.clone();
        let thread = thread::spawn(move || {
            for table in thread_lookup_tables.iter() {
                print!("{value}->");
                value = table.lookup(value);
            }
            println!("{value} {}", lowest_loc_ref.load(Ordering::Relaxed));
            if value < lowest_loc_ref.load(Ordering::Relaxed) {
                lowest_loc_ref.store(value, Ordering::Relaxed);
            }
        });
        threads.push(thread);
    }
    for thread in threads {
        thread.join().unwrap();
    }
    println!("Value: {}", lowest_loc.load(Ordering::Relaxed));
}

fn part_two() {
    let mut seed_data: String = String::default();
    let mut lookup_tables: Vec<Table> = Vec::new();

    let mut input_iter = PUZZLE_INPUT.lines();
    let mut not_done = true;
    while not_done {
        let line = if let Some(line) = input_iter.next() {
            line
        } else {
            not_done = false;
            continue;
        };
        
        if line.contains("seeds: ") {
            println!("getting seeds...");
            let (_, seeds_str) = line.split_once(": ").unwrap();
            seed_data = seeds_str.to_string();
        }

        if line.contains("map") {
            print!("building {}", line);
            let mut map_not_done = true;
            let mut map_table = Table::new();
            while map_not_done {
                if let Some(map_data) = input_iter.next() {
                    if map_data.is_empty() {
                        map_not_done = false;
                        continue;
                    } 
                    map_table.add_table(map_data);
                } else {
                    not_done = false;
                    break;
                }
            }
            println!("done");
            lookup_tables.push(map_table);
        }
    }

    println!("mapping seed to loc...");
    let lowest_loc = Arc::new(AtomicUsize::new(usize::MAX));
    let mut threads = Vec::new();
    let arc_lookup_tables = Arc::new(lookup_tables);

    let mut seed_iter = seed_data.split_whitespace();
    let seeds_not_done = true;
    while seeds_not_done {
        let mut seed_starting_number = 0;
        if let Some(seed_start) = seed_iter.next() {
            seed_starting_number = seed_start.parse().unwrap();
        } else {
            break;
        }
        let seed_count: usize = seed_iter.next().unwrap().parse().unwrap();
        for s in seed_starting_number..seed_starting_number + seed_count {
            let mut value = s.clone();
            let thread_lookup_tables = arc_lookup_tables.clone();
            let lowest_loc_ref = lowest_loc.clone();
            let thread = thread::spawn(move || {
                for table in thread_lookup_tables.iter() {
                    //print!("{value}->");
                    value = table.lookup(value);
                }
                //println!("{value} {}", lowest_loc_ref.load(Ordering::Relaxed));
                if value < lowest_loc_ref.load(Ordering::Relaxed) {
                    lowest_loc_ref.store(value, Ordering::Relaxed);
                }
            });
            threads.push(thread);
        }
    }
    
    for thread in threads {
        thread.join().unwrap();
    }
    println!("Value: {}", lowest_loc.load(Ordering::Relaxed));
}

fn main() {
    part_one();
    part_two();
}
