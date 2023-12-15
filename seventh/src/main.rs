mod input;
use input::*;
use std::collections::HashMap;

fn get_card_value(card: char) -> usize {
    match card {
        '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => card.to_digit(10).unwrap() as usize,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("what card is this?"),
    }
}

fn get_hand_rank(hand_str: &str) -> usize {
    let hand = hand_str.chars().collect::<Vec<char>>();
    let mut card_counts: HashMap<char, usize> = HashMap::new();
    for card in &hand {
        if card_counts.contains_key(card) {
            let c = card_counts.get_mut(card).unwrap();
            *c += 1;
        } else {
            card_counts.insert(*card, 1);
        }
    }
    let mut pair = false;
    let mut three = false;
    let mut rank = 0;
    for (_, count) in card_counts {
        match count {
            5 => return 6,
            4 => return 5,
            3 => {
                if pair {
                    // full house
                    return 4;
                } else {
                    three = true;
                    rank = 3;
                }
            },
            2 => {
                if three {
                    // full house
                    return 4;
                } else if pair {
                    return 2;
                } else {
                    pair = true;
                    rank = 1;
                }
            },
            1 => if three == false && pair == false {
                rank = 0;
            },
            _ => panic!("ummmm no?"),
        }
    }
    rank
}

#[derive(Debug, Clone)]
pub struct Hand {
    hand: String,
    bid: usize,
}

fn part_one() {
    let mut ranked_hands: Vec<Vec<Hand>> = Vec::new();
    for _ in 0..7 {
        ranked_hands.push(Vec::new());
    }
    let mut sorted_ranks = ranked_hands.clone();
    for line in PUZZLE_INPUT.lines() {
        // get the hand and the bid
        let(current_hand_str, current_bid) = line.split_once(' ').unwrap();
        // derive the hand rank based on the hank
        let current_hand_rank = get_hand_rank(current_hand_str);
        // create Hand struct based on rank and hand
        let current_hand = Hand {
            hand: current_hand_str.to_string(),
            bid: current_bid.parse().unwrap(),
        };

        ranked_hands.get_mut(current_hand_rank).expect(format!("{} {:?}", current_hand_rank, current_hand).as_str()).push(current_hand);
    }

    let mut rank_index = 0;
    for rv in &ranked_hands {
        let mut sorted_hands: Vec<Hand> = Vec::new();
        for current_hand in rv {
            if sorted_hands.len() == 0 {
                sorted_hands.push(current_hand.clone());
                continue;
            }
            let mut card_check_done = false;
            let mut current_is_higher = false;
            for hand_index in 0..sorted_hands.len() {
                if card_check_done {
                    break;
                }
                // get a vec of chars for the current hand we are looking at
                let current_cards: Vec<char> = current_hand.hand.chars().collect();
                // get a vec of chars for the current hand in the hand rank vec
                let vec_cards: Vec<char> = sorted_hands[hand_index].hand.chars().collect();

                for card_index in 0..5 {
                    let current_card = current_cards[card_index];
                    let existing_card = vec_cards[card_index];
                    let current_card_value = get_card_value(current_card);
                    let existing_card_value = get_card_value(existing_card);
                    
                    if current_card_value == existing_card_value {
                        continue;
                    } else if current_card_value > existing_card_value {
                        current_is_higher = true;
                        break;
                    } else {
                        sorted_hands.insert(hand_index, current_hand.clone());
                        card_check_done = true;
                        break;
                    }
                }
                if hand_index == sorted_hands.len() - 1 && current_is_higher {
                    sorted_hands.push(current_hand.clone());
                }
                
            }
        }
        sorted_ranks[rank_index] = sorted_hands;   
        rank_index += 1;
    }

    let flattened = sorted_ranks.into_iter().flatten().collect::<Vec<Hand>>();
    let mut total_bid_value = 0;

    for i in 0..flattened.len() {
        let bid_value = (i + 1) * flattened[i].bid;
        total_bid_value += bid_value;
    }
    
    println!("Value: {total_bid_value}");
}

fn part_two() {

}

fn main() {
    part_one();
    part_two();
}
