mod input;
use crate::input::*;

fn part_one() {
    let mut card_points: Vec<usize> = Vec::new();

    for line in PUZZLE_INPUT.lines() {
        let (_, card_numbers) = line.split_once(": ").unwrap();
        let (winning_numbers, card_numbers) = card_numbers.split_once(" | ").unwrap();
        let winning_numbers: Vec<usize> = winning_numbers.split_whitespace().map(|x| x.parse::<usize>().unwrap() ).collect();
        let card_numbers: Vec<usize> = card_numbers.split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect();
        let mut points = 0;
        for card_number in &card_numbers {
            for winning_number in &winning_numbers {
                if card_number == winning_number {
                    match points {
                        0 => points += 1,
                        _ => points *= 2,
                    }
                }
            }
        }
        card_points.push(points);
    }
    println!("Value: {}", card_points.iter().sum::<usize>());
}

#[derive(Debug, Clone)]
struct ScratchCard {
    card_number: usize,
    winning_card: bool,
    matching_numbers: usize,
}

fn part_two() {
    let mut cards: Vec<ScratchCard> = Vec::new();
    let mut card_count = 0;
    // build a card table
    let mut card_index = 0;
    for line in PUZZLE_INPUT.lines() {
        let (_, card_numbers) = line.split_once(": ").unwrap();
        let (winning_numbers, card_numbers) = card_numbers.split_once(" | ").unwrap();
        let winning_numbers: Vec<usize> = winning_numbers.split_whitespace().map(|x| x.parse::<usize>().unwrap() ).collect();
        let card_numbers: Vec<usize> = card_numbers.split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect();
        let mut card = ScratchCard {
            card_number: card_index + 1,
            winning_card: false,
            matching_numbers: 0,
        };        
        for card_number in &card_numbers {
            for winning_number in &winning_numbers {
                if card_number == winning_number {
                    card.winning_card = true;
                    card.matching_numbers += 1;
                }
            }
        }
        cards.push(card);
        card_index += 1;
    }

    let cards_backup = cards.clone();
    let mut new_cards_added = true;
    while new_cards_added {
        let mut extra_cards = Vec::new();
        for card in &cards {
            card_count += 1;
            if card.winning_card {
                new_cards_added = true;
                let n = card.card_number;
                let wn = n + card.matching_numbers;
                let new_cards = &cards_backup[n..wn];
                for i in new_cards {
                    let winning_card = i;
                    extra_cards.push(winning_card.clone());
                }
            }
        }
        if extra_cards.len() > 0 {
            cards.clear();
            cards.append(&mut extra_cards);
        } else {
            new_cards_added = false;
        }
    }
    println!("Value: {}", card_count);
}
 
fn main() {
    part_one();
    part_two();
}
