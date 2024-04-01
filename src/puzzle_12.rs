use core::{panic, num};
use std::collections::HashMap;
use std::{fs};
use std::io::{BufWriter, Write};

use std::time::{Duration, SystemTime};

pub fn solve() { 
    solve_part_1();
    // solve_part_2();
    test_new_method();
}

pub fn solve_part_1() {
    let input_string = fs::read_to_string("./data/puzzle_12_test_easy.txt").expect("File read error");
    let lines = input_string.lines();

    let mut total_counter = 0;

    let file = fs::File::create("puzzle_12_1x.txt").unwrap();
    let mut writer = BufWriter::new(file);
    for (l, line) in lines.enumerate() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let sequence: Vec<char> = parts.first().unwrap().chars().collect();
        let part_lengths: Vec<usize> = parts.last().unwrap().split(",").into_iter().map(|x| x.parse().unwrap()).collect();

        // let line_score = count_all_valid_combinations(&sequence, &part_lengths, 0, 0);
        println!("{} - {:?}", String::from_iter(sequence.clone()), part_lengths);
        let line_score = count_all_valid_combinations(&sequence, &part_lengths, 0, 0);
        // let line_score = count_all_valid_combinations_3(&sequence, &part_lengths, 0);
        println!("{}", line_score);
        // writer.write_all(format!("{} - {}\n", line, line_score).as_bytes());
        total_counter += line_score; 
        // break;
    }
    println!("Puzzle 12 game sum = {}", total_counter);
}



pub fn test_new_method() {
    let input_string = fs::read_to_string("./data/puzzle_12.txt").expect("File read error");
    let lines = input_string.lines();

    let mut total_counter = 0;

    let factor = 4;
    // let file = fs::File::create("puzzle_12_1x.txt").unwrap();
    // let mut writer = BufWriter::new(file);

    let mut memorization_hashmap: HashMap<(Vec<char>, Vec<usize>), u64> = HashMap::new();

    let file = fs::File::create("puzzle_12_4x.txt").unwrap();
    let mut writer = BufWriter::new(file);
    for (l, line) in lines.enumerate() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let sequence: Vec<char> = parts.first().unwrap().chars().collect();
        let part_lengths: Vec<usize> = parts.last().unwrap().split(",").into_iter().map(|x| x.parse().unwrap()).collect();

        let mut unfolded_sequence = sequence.clone(); 
        let mut unfolded_part_lengths = part_lengths.clone(); 
        for k in 0..factor {
            unfolded_sequence.push('?');
            unfolded_sequence.extend(sequence.clone());
            unfolded_part_lengths.extend(part_lengths.clone());
        }

        println!("-- {} - {:?}", String::from_iter(sequence.clone()), part_lengths);
        let mut line_score = 0;
        trim_sequence(&mut unfolded_sequence);
        let t = SystemTime::now();
        for k in 0..1 {
            line_score = count_all_valid_combinations_4(&unfolded_sequence, &unfolded_part_lengths, &mut memorization_hashmap);
        }
        // writer.write_all(format!("{} - {}\n", line, line_score).as_bytes());
        println!("---- Solved in: {:?}", SystemTime::now().duration_since(t));
        println!("Score   =   {}", line_score);
        println!("");
        total_counter += line_score; 

    }
    println!("Puzzle 12 game sum = {}", total_counter);
}


pub fn solve_part_2() {
    let input_string = fs::read_to_string("./data/puzzle_12.txt").expect("File read error");
    let lines = input_string.lines();

    let factor = 4;
    let file = fs::File::create(format!("puzzle_12_{}x.txt", factor)).unwrap();
    let mut writer = BufWriter::new(file);
    let mut total_counter = 0;

    let mut total_time_us = Duration::from_micros(0);

    for (l, line) in lines.enumerate() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let sequence: Vec<char> = parts.first().unwrap().chars().collect();
        let part_lengths: Vec<usize> = parts.last().unwrap().split(",").into_iter().map(|x| x.parse().unwrap()).collect();
        
        let mut unfolded_sequence = sequence.clone(); 
        let mut unfolded_part_lengths = part_lengths.clone(); 
        for k in 0..factor {
            unfolded_sequence.push('?');
            unfolded_sequence.extend(sequence.clone());
            unfolded_part_lengths.extend(part_lengths.clone());
        }
        // let broken_candidates = get_broken_candidates(&sequence);
        
        println!("{} - {:?}", String::from_iter(unfolded_sequence.clone()), unfolded_part_lengths);
        
        let t = SystemTime::now();
        let line_score = count_all_valid_combinations_3(&unfolded_sequence, &unfolded_part_lengths,  0);
        total_time_us += SystemTime::now().duration_since(t).unwrap_or_default();
        // println!("Score: {}", line_score);
        println!("{} - {}", line, line_score);
        println!("Average time: {:?}", total_time_us.checked_div(l as u32).unwrap_or(Duration::from_micros(0)));
        println!("{}", line_score);
        writer.write_all(format!("{} - {}\n", line, line_score).as_bytes());
        total_counter += line_score; 
        // break;

    }
    println!("Puzzle 12 B game sum = {}", total_counter);
}

pub fn test_factor(factor: u64) -> u64 {
    let input_string = fs::read_to_string("./data/puzzle_12_test.txt").expect("File read error");
    let lines = input_string.lines();

    let file = fs::File::create(format!("puzzle_12_{}x.txt", factor)).unwrap();
    let mut writer = BufWriter::new(file);
    let mut total_counter = 0;

    for (l, line) in lines.enumerate() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let sequence: Vec<char> = parts.first().unwrap().chars().collect();
        let part_lengths: Vec<usize> = parts.last().unwrap().split(",").into_iter().map(|x| x.parse().unwrap()).collect();
        
        let mut unfolded_sequence = sequence.clone(); 
        let mut unfolded_part_lengths = part_lengths.clone(); 
        for k in 0..factor {
            unfolded_sequence.push('?');
            unfolded_sequence.extend(sequence.clone());
            unfolded_part_lengths.extend(part_lengths.clone());
        }
        // let broken_candidates = get_broken_candidates(&sequence);
        
        // println!("{} - {:?}", String::from_iter(unfolded_sequence.clone()), unfolded_part_lengths);
        let line_score = count_all_valid_combinations(&unfolded_sequence, &unfolded_part_lengths, 0, 0);
        // println!("Score: {}", line_score);
        // println!("{} - {}", line, line_score);
        println!("{}", line_score);
        writer.write_all(format!("{} - {}\n", line, line_score).as_bytes());
        total_counter += line_score; 
        // break;
    }
    println!("Puzzle 12 B game sum = {}", total_counter);
    total_counter
}


fn trim_sequence(sequence: &mut Vec<char>) {
    while sequence.last().unwrap() == &'.' {
        sequence.pop();
    }

    let mut start_index = 0;
    for c in 0..sequence.len() {
        if sequence[c] == '.' {
            start_index = c + 1;
        }
        else {
            break;
        }
    }
    // println!("Start index = {}", start_index);
    sequence.drain(0..start_index);
}

fn check_is_all_hashtag(sequence: &[char]) -> bool {
    for c in sequence {
        if c != &'#' {
            return false;
        }
    }

    true
}
fn count_all_valid_combinations_3(sequence: &Vec<char>, part_lengths: &Vec<usize>, part_index: usize) -> u64 {
    // println!("");

    let mut counter = 0;
    let part_length = part_lengths[part_index] as usize;
    // let end_index = sequence.len().checked_sub(nr_part_chars).unwrap_or_default() + 1;
    // println!("Sum of parts up until index {} = {}", part_index, part_lengths[part_index.checked_sub(1).unwrap_or_default()..].into_iter().sum::<usize>());
    let min_nr_characters_for_remaining_parts = (part_lengths[part_index..].into_iter().sum::<usize>() + part_lengths[part_index..].len() as usize).checked_sub(1).unwrap_or_default();

    // println!("{} - part length: {}", String::from_iter(sequence.clone()), part_length);
    for part_start in 0..sequence.len() {
        // if memorization_hashmap.contains_key((sequence, ))
        // remainder can't fit remaining parts
        if sequence.len() - part_start < min_nr_characters_for_remaining_parts {
            break;
        }

        let part_end = part_start + part_length;
        let sequence_part_candidate = &sequence[part_start..part_end];

        // candidate consists of ? and # only and is of length part_length
        if sequence_part_candidate.contains(&'.') {
            continue;
        }

        // Last part
        if part_index == part_lengths.len() - 1 && !sequence[part_end..].contains(&'#') {
            counter += 1;
            continue;
        }

        // part finalizes string and is thus valid
        if part_end == sequence.len() {
            counter += 1;
            break;
        }

        if part_start >= 1 && sequence[part_start-1] == '#' {
            break;
        }

        // not the last part, but part location is chosen, so all next combinations have to be generated recursively
        if sequence[part_end] == '.' || sequence[part_end] == '?' {
            if part_index < part_lengths.len() - 1 { 
                // create new slice
                let mut new_sequence = Vec::new();
                new_sequence.extend_from_slice(&sequence[part_start+part_length+1..]); // + 1 for seperating ./?

                let mut remaining_parts_list = Vec::new();
                remaining_parts_list.extend_from_slice(&part_lengths[part_index+1..]); // + 1 for seperating ./?

                // trim dots away
                trim_sequence(&mut new_sequence);
                
                // find all combinations
                let nr_combinations = count_all_valid_combinations_3(&new_sequence, &part_lengths, part_index + 1);
                counter += nr_combinations;

                // if nr_combinations > 1 {
                    // println!("Nr combinations for: {}", String::from_iter(new_sequence));
                    // println!("Remaining parts: {:?}", &part_lengths[part_index + 1..]);
                    // println!("{}", nr_combinations);
                // }
            }
        }

        // when passing a # without accounting it for with a part, all following combinations are invalid
        // if sequence.first().unwrap() == &'#' {
        //     break;
        // }
    }
    // if part_index > 2 && part_index < 5 {
    //     println!("Sequence {:?} - {:?} - {} ", String::from_iter(sequence), &part_lengths[part_index..], counter);        
    // }
    // println!("Counter: {}", counter);
    counter
}

fn count_all_valid_combinations_4(sequence: &Vec<char>, part_lengths: &Vec<usize>, memorization_hashmap: &mut HashMap<(Vec<char>, Vec<usize>), u64>) -> u64 {
    // println!("");

    let mut counter = 0;
    let part_length = part_lengths.first().unwrap();
    let min_nr_characters_for_remaining_parts = (part_lengths.into_iter().sum::<usize>() + part_lengths.len() as usize).checked_sub(1).unwrap_or_default();

    // println!("Checking: \n{}", String::from_iter(sequence.clone()));
    for part_start in 0..sequence.len() {
        let solution = memorization_hashmap.get(&(sequence.clone(), part_lengths.clone()));
        
        if part_start >= 1 && sequence[part_start-1] == '#' {
            break;
        }
        if let Some(nr_combinations) = solution  {
            return *nr_combinations;
        }

        // remainder can't fit remaining parts
        if sequence.len() - part_start < min_nr_characters_for_remaining_parts {
            break;
        }

        let part_end = part_start + part_length;
        let sequence_part_candidate = &sequence[part_start..part_end];

        // candidate consists of ? and # only and is of length part_length
        if sequence_part_candidate.contains(&'.') {
            continue;
        }


        // Last part
        if part_lengths.len() == 1 && !sequence[part_end..].contains(&'#') {
            counter += 1;
            // println!("Valid combination: \n{:width$}", String::from_iter(sequence_part_candidate.clone()), width=part_start);
            continue;
        }

        // part finalizes string and is thus valid
        if part_end == sequence.len() {
            counter += 1;
            // println!("Valid combination: \n{:width$}", String::from_iter(sequence_part_candidate.clone()), width=part_start);
            break;
        }

        // not the last part, but part location is chosen, so all next combinations have to be generated recursively
        if sequence[part_end] == '.' || sequence[part_end] == '?' {
            if part_lengths.len() > 1 { 
                // create new slice
                let mut new_sequence = Vec::new();
                new_sequence.extend_from_slice(&sequence[part_start+part_length+1..]); // + 1 for seperating ./?

                let mut remaining_parts_list = Vec::new();
                remaining_parts_list.extend_from_slice(&part_lengths[1..]); // + 1 for seperating ./?

                // trim dots away
                trim_sequence(&mut new_sequence);
                
                // find all combinations
                let nr_combinations = count_all_valid_combinations_4(&new_sequence, &remaining_parts_list, memorization_hashmap);
                memorization_hashmap.insert((new_sequence.clone(), remaining_parts_list.clone()), nr_combinations);
                counter += nr_combinations;

                // println!("Sequence: {}, parts: {:?}\nScore:{}\n\n", String::from_iter(new_sequence.clone()), remaining_parts_list, nr_combinations);
            }
        }
    }
    counter
}
























fn count_all_valid_combinations_2(sequence: &Vec<char>, part_lengths: &Vec<usize>, start_index: usize, part_index: usize) -> u64 {
    let mut total = 0; 


    for (p, part_length) in part_lengths.into_iter().enumerate() {
        let start_index = part_lengths[0..p].iter().sum::<usize>() + part_lengths[0..p].len() as usize;
        let stop_index = sequence.len().checked_sub(part_lengths[p..].into_iter().sum::<usize>() + part_lengths[p..].len() as usize).unwrap_or(sequence.len());

        println!("Checking from {} to {} for sequence {:?}", start_index, stop_index, sequence);

        let sequence_to_check = String::from_iter(&sequence[start_index..stop_index]);
        let sequences = sequence_to_check.split('.');

        let mut nr_combinations = 0;

        for seq in sequences {
            let seq_points = seq.len().checked_sub(*part_length).unwrap_or(0) + 1;
            nr_combinations += seq_points;
        }
        

        // for c in start_index..stop_index {

        // } 
        
    }

    total
}

fn count_all_valid_combinations(sequence: &Vec<char>, part_lengths: &Vec<usize>, start_index: usize, part_index: usize) -> u64 {
    // println!("");
    let mut counter = 0;
    // if part_index == 0 {
    //     println!("Checking parts: {:?}", part_lengths);
    // }
    // for (p, part_length) in part_lengths.iter().enumerate() {
    // let first_part = part_lengths.first().unwrap();
    let nr_part_chars = part_lengths[part_index] as usize;
    // println!("{} - {} parts - {} - {}", String::from_iter(sequence), nr_part_chars, start_index, sequence.len());
    
    if start_index + nr_part_chars <= sequence.len() {
        let end_index = sequence.len().checked_sub(nr_part_chars).unwrap() + 1;
        // println!("Indexes {} to {}", start_index, end_index);
        for k in start_index..end_index {
            // println!("k = {}, start_index={}", k, start_index);
            if k != 0 {
                if sequence[k-1] == '#' {
                    // println!("Before");
                    continue;
                }
            }

            if k + nr_part_chars < sequence.len() {
                if sequence[k + nr_part_chars] == '#' {
                    // println!("After");
                    continue;
                }
            }

            if sequence[k..k + nr_part_chars].contains(&'.') {
                // println!("{}..{}", k, k+nr_part_chars);
                // println!("Contains .: {:?}", &sequence[k..k + nr_part_chars]);
                continue;
            }
            
            if part_index < part_lengths.len()-1 {
            // if part_lengths.len() > 1 {

                let mut new_sequence = sequence.clone();

                for l in k..k+nr_part_chars {
                    new_sequence[l] = '#';   
                }
                
                for l in 0..k {
                    if new_sequence[l] == '?' {
                        new_sequence[l] = '.';
                    } 
                }
        
                counter += count_all_valid_combinations(&new_sequence, &part_lengths, k + nr_part_chars, part_index + 1);
            }
            else {
                
                // let t = SystemTime::now();
                let mut new_sequence = sequence.clone();
        
                for l in k..k+nr_part_chars {
                    new_sequence[l] = '#';   
                }
                for l in 0..new_sequence.len() {
                    if new_sequence[l] == '?' {
                        new_sequence[l] = '.';
                    }
                }
                // println!("Time to fill in: {:?} sec", SystemTime::now().duration_since(t));
                // let t = SystemTime::now();
                if verify_validity_sequence(&new_sequence, &part_lengths) {
                    // println!("{} -- valid combo", String::from_iter(new_sequence));
                    counter += 1;
                }
                // println!("Time to verify: {:?} sec", SystemTime::now().duration_since(t));
            }
        }
    }
    // println!("Counter: {}", counter);
    counter
}


fn verify_validity_sequence(sequence: &Vec<char>, part_lengths: &[usize]) -> bool {
    // println!("Verifying");
    let mut is_correct = true;
    let mut k = 0;

    if sequence.contains(&'?') {
        panic!("Invalid sequence! Still contains ?: {}", String::from_iter(sequence));
    }

    let mut sequence_length = 0;
    let mut part_length_index = 0;
    // println!("Checking for validity:");
    // println!("{}", String::from_iter(sequence));
    for k in 0..sequence.len() {
        if sequence[k] == '#' {
            // start counting sequence
            // print!("{}", sequence[k]);
            sequence_length += 1;

            if part_length_index >= part_lengths.len() {
                return false;       
            }
        }
        else {
            if sequence_length > 0 {
                // otherwise, reset sequence

                if part_length_index < part_lengths.len() && sequence_length != part_lengths[part_length_index] {
                    // println!("{} : Sequence length {} != {}", k, sequence_length, part_lengths[part_length_index]);
                    return false;
                }
                // println!();
                part_length_index += 1;
                sequence_length = 0;
            }
        }
    }
    if sequence_length > 0 {
        // otherwise, reset sequence
        if sequence_length != part_lengths[part_length_index] {
            // println!("{} : Sequence length {} != {}", k, sequence_length, part_lengths[part_length_index]);
            return false;
        }
        // println!();
        part_length_index += 1;
        sequence_length = 0;
    }
    if part_length_index != part_lengths.len() {
        // println!("Not valid");
        return false;
    }

    if !is_correct {
        // println!("Not valid");
    }

    is_correct
}
