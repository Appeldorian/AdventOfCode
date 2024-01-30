use core::{panic, num};
use std::{fs, io::empty, thread, time};
use ndarray::prelude::*;

pub fn solve() { 
    let input_string = fs::read_to_string("./data/puzzle_12.txt").expect("File read error");
    let lines = input_string.lines();

    let nr_valid_combinations = 0;
    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let sequence: Vec<char> = parts.first().unwrap().chars().collect();
        let broken_lengths: Vec<u32> = parts.last().unwrap().split(",").into_iter().map(|x| x.parse().unwrap()).collect();
        let broken_candidates = get_broken_candidates(&sequence);
        
        let mut start_positions = vec![0; broken_lengths.len()];
        let mut nr_combinations = 0;
        let mut start_position = 0;
        let max_start_position = broken_lengths.iter().count() + broken_lengths.len() - 1; // assuming min 1 dot between each sequence
        for k in 0..max_start_position {
            let mut new_sequence = sequence.clone();
            // new_sequence = set_broken_part(&new_sequence, &broken_lengths.first().unwrap(), &start_position);

        }

        // recursieve functie hier zou cool zijn die: 
        // - alle combinaties probeert met de eerste broken_length in een vector.
        // - deze combinaties telt en dan het element verwijdert uit de vector of VecDeque
        // 

        // loop  {
        //     let mut combination: String = sequence.iter().collect();
        //     let mut broken_length_index = 0;         
            
        //     for (l, length) in broken_lengths.into_iter().enumerate() {
        //         let new_combination = set_broken_part(&combination, length, start_positions[l]);
        //         if new_combination.is_some() && verify_validity_sequence(&new_combination.unwrap(), &broken_lengths) {
        //             nr_combinations += 1;
        //         }
        //     }
        // }

    }
}

fn get_broken_candidates (full_sequence: &Vec<char>) -> Vec<Vec<char>> {
    let mut candidate_sequences = Vec::new();
    let mut broken_sequence = Vec::new(); 
    for c in full_sequence {
        if c == &'.' {
            if !broken_sequence.is_empty() {
                candidate_sequences.push(broken_sequence);
                broken_sequence = Vec::new(); 
            }
            continue;
        }

        broken_sequence.push(c.clone());      
    }
    if !broken_sequence.is_empty() {
        candidate_sequences.push(broken_sequence);
    }

    candidate_sequences
}

fn set_broken_part(sequence: &Vec<char>, nr_parts: u32, start_position: usize) -> Option<String> {
    // let mut result_string: Result<String>;
    let mut new_sequence: String = sequence.iter().collect();
    let characters: Vec<char> = new_sequence.chars().collect();
    let mut start_count = 0;
    for k in 0..(characters.len() - nr_parts as usize) {
        let mut part = &characters[k..k + nr_parts as usize];
        if part.contains(&'.') {
            continue;
        }
        if start_count >= start_position {
            let part_string: String = part.iter().collect();
            let last_character = k + nr_parts as usize;
            if last_character > characters.len() {
                return None;
            }
            let range = k..last_character;
            
            new_sequence.replace_range(range, part_string.as_str());

            if last_character != characters.len() {
                new_sequence.replace_range(last_character..last_character+1, ".");
            }
            break;
        }
        else {
            start_count += 1;
        }

    }

    Some(new_sequence)
}

fn verify_validity_sequence(sequence: &Vec<char>, broken_lengths: &[u32]) -> bool {
    let mut is_correct = true;
    let mut k = 0;

    let mut sequence_length = 0;
    let mut broken_length_index = 0;
    for k in 0..sequence.len() {
        if sequence[k] == '#' {
            // start counting sequence
            sequence_length += 1;
        }
        else {
            // otherwise, reset sequence
            sequence_length = 0;
        }

        if broken_length_index < broken_lengths.len() {      
            if sequence_length > broken_lengths[broken_length_index] {
                // If sequence is longer than what it should be -> invalid
                is_correct = false;
                break;
            }

            if sequence_length != 0 && sequence[k] != '#' {
                // If sequence is longer than what it should be -> invalid
                if sequence_length == broken_lengths[broken_length_index] {
                    broken_length_index += 1;
                }
            }
        } else {
            if sequence[k] == '#' {
                // If  all sequences have been accounted for and yet another # shows up -> false
                is_correct = false;
                break;
            }
        } 
    }
    
    is_correct
}

fn count_all_valid_combinations(sequence: &Vec<char>, broken_lengths: Vec<u32>) -> u32 {
    let mut counter = 0;
    if broken_lengths.len() > 1 {
        counter += count_all_valid_combinations(sequence, broken_lengths);
    }
    for k in 0..sequence.len()-broken_length as usize {
        if !sequence[k..k+broken_length as usize].contains(&'.') {
            if k != 0 && sequence[k-1] == '#' {
                continue;
            }

            if (k + broken_length as usize) < sequence.len() && sequence[k+1] == '#' {
                continue;
            }

            counter += 1;
        }
    }

    counter
}