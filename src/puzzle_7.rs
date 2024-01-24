use core::panic;
use std::{fs, collections::HashMap, cmp, fmt, time, hash::Hash, vec};

// TODO: Encodeer ook waardes op basis van welke 5/4/3 of a kind etc...
// Momenteel enkel score voor bvb full house, maar niet hoger voor QQQKK als 33222 bvb

pub fn solve() { 
    let input_string = fs::read_to_string("./data/puzzle_7.txt").expect("File read error");
    let line_iterator = input_string.lines();

    let mut hand_strings = Vec::new();
    let mut hand_values = Vec::new();
    let mut hand_values_with_joker= Vec::new();
    let mut bet_values: Vec<u64> = Vec::new();

    for line in line_iterator {
        let hand_string = line.split(" ").nth(0).unwrap();
        let bet_value = line.split(" ").nth(1).unwrap();
        let mut hand = Hand::new(hand_string);
        let hand_value = hand.get_hand_value();
        hand.set_joker_option(true);
        let hand_value_with_joker = hand.get_hand_value();

        hand_strings.push(hand_string);
        hand_values.push(hand_value);
        hand_values_with_joker.push(hand_value_with_joker);
        bet_values.push(bet_value.parse().unwrap());
    }

    let sort_indices = argsort(&hand_values);
    // let sorted_hand_values = apply_indices(&hand_values, &sort_indices);
    // let sorted_bet_values = apply_indices(&hand_values, &sort_indices);

    let mut game_sum = 0;
    for (k, index) in sort_indices.into_iter().enumerate() {
        let rank_value = (k as u64 + 1) * bet_values[index];
        game_sum += rank_value;
        
        println!("Rank {}: {} {} => {} => {}", k, hand_strings[index], bet_values[index], hand_values[index], rank_value);
    }
    println!("Puzzle 7 sum = {}", game_sum);

    let sort_indices_with_joker = argsort(&hand_values_with_joker);
    // let sorted_hand_values = apply_indices(&hand_values, &sort_indices);
    // let sorted_bet_values = apply_indices(&hand_values, &sort_indices);

    let mut game_sum = 0;
    for (k, index) in sort_indices_with_joker.into_iter().enumerate() {
        let rank_value = (k as u64 + 1) * bet_values[index];
        game_sum += rank_value;
        
        println!("Rank {}: {} {} => {} => {}", k, hand_strings[index], bet_values[index], hand_values_with_joker[index], rank_value);
    }
    println!("Puzzle 7 bonus sum = {}", game_sum);

}



#[derive(PartialEq)]
struct Hand {
    cards: [char; 5],
    with_joker: bool,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        if self.with_joker {
            
        }
        let self_hand_type = self.get_hand_type();
        let other_hand_type = other.get_hand_type();
        let mut result = Some(cmp::Ordering::Equal);
        if self_hand_type > other_hand_type {
            result = Some(cmp::Ordering::Greater);
        }
        else if self_hand_type < other_hand_type {
            result = Some(cmp::Ordering::Less)
        }
        else {
            for card_index in 0..self.cards.len() {
                if self.cards[card_index] > other.cards[card_index] {
                    result = Some(cmp::Ordering::Greater);
                }
                else if self.cards[card_index] < other.cards[card_index] {
                    result = Some(cmp::Ordering::Less);
                }
            }
        }
        result // If it passes everything, both hands are equal
    }
}

impl Hand {
    fn new(hand_string: &str) -> Self {
        assert!(hand_string.chars().count() == 5, "Hand string not equal to length 5!");
        
        let mut cards: [char; 5] = [' '; 5];
        for k in 0..5 {
            cards[k] = hand_string.chars().nth(k).unwrap();
        }

        Hand { cards: cards, with_joker: false}
    }

    fn set_joker_option(&mut self, joker_on: bool) {
        self.with_joker = joker_on;
    }

    
    fn get_card_value(&self, card: &char, use_joker_value: bool) -> u64 {
        let value = match card.to_digit(10) {
            Some(value) => value,
            None => match card {
                    'A' => 14,
                    'K' => 13,
                    'Q' => 12,
                    'J' =>  { 
                                let mut value = 11;
                                if use_joker_value {
                                    value = 1;
                                } 
                                value
                            },
                    'T' => 10,
                    _ => panic!("wrong sign!"),
                }
        } as u64;

        if value == 0{
            panic!("Unknown card value: {}", card);
        }

        value
    }

    fn get_hand_type(&self) -> u64 {
        let mut card_count = HashMap::new();
        for card in self.cards {
            let nr_cards_like_card = card_count.get(&card);
            let count = match nr_cards_like_card {
                Some(value) => *value + 1,
                None => 1, 
            };
            card_count.insert(card, count);
        }

        if self.with_joker {
            if card_count.contains_key(&'J') {
                let mut max_value_key = ' ';
                let mut max_value = 0;
                for key in card_count.keys() {
                    if card_count[key] >= max_value && *key != 'J' {
                        max_value_key = *key;
                        max_value = card_count[key];
                    }
                }
                
                if max_value > 0 {              
                    card_count.insert(max_value_key, card_count[&'J'] + max_value);
                    card_count.remove(&'J'); 
                }
            }
        }
        let cards_found = card_count.keys();
        let nr_cards_found = cards_found.len();
        let hand_type = match nr_cards_found {
            5 => 1,
            4 => 2,
            3 =>  { 
                    let max_count = card_count.values().max().unwrap();
                    match max_count {
                        2 => 3,
                        3 => 4,
                        _ => panic!("Bad max count: {}", max_count)
                    }
                },
            2 => {
                    let max_count = card_count.values().max().unwrap();
                    match max_count {
                        4 => 6,
                        3 => 5,
                        _ => panic!("Bad max count: {}", max_count)
                    }
            },
            1 => 7,
            _ => panic!("Bad nr cards found: {}", nr_cards_found)
        };
        hand_type
    }

    
    fn get_hand_value(&self) -> u64 {
        let hand_type = self.get_hand_type();

        let mut tie_breaker_value = 0;
        for (index, card_value) in self.cards.iter().enumerate() {
            tie_breaker_value += self.get_card_value(&card_value, true) * 15_u64.pow(5-(index as u32)); 
        }
        hand_type*15_u64.pow(6) + tie_breaker_value
    }

    

    
}

pub fn argsort<T: Ord>(data: &[T]) -> Vec<usize> {
    let mut indices = (0..data.len()).collect::<Vec<_>>();
    indices.sort_by_key(|&i| &data[i]);
    // indices.reverse();
    indices
}

pub fn apply_indices<T: Copy>(vector: &Vec<T>, indices: &Vec<usize>) -> Vec<T> {
    assert_eq!(vector.len(), indices.len());

    let mut sorted_vec = Vec::new();
    for index in indices.clone()  {
        sorted_vec.push(vector[index]);
    }

    sorted_vec
}
