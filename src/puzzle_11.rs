use core::{panic, num};
use std::{fs, io::empty, thread, time};
use ndarray::prelude::*;

pub fn solve() { 
    let input_string = fs::read_to_string("./data/puzzle_11.txt").expect("File read error");
    // println!("{}", input_string);
    let mut character_map = Vec::new();
    // for c in input_string.chars() {
    //     println!("{}", c);
    // }
    let split = input_string.split_whitespace();
    // println!("{:?}", split);
    for line in split {
        let line_chars: Vec<char> = line.chars().collect();
        // println!("{:?}", line_chars);
        character_map.push(line_chars);
    }


    
    // let mut star_map = Array2::<u32>::zeros((character_map.len(), character_map.first().unwrap().len()));
    let mut empty_rows = Vec::new();
    let mut empty_cols = Vec::new();
    for c in 0..character_map.first().unwrap().len() {
        let mut no_galaxies = true;
        for r in 0..character_map.len() {
            if character_map[r][c] == '#' {
                no_galaxies = false;
                break;
            }
        }
        
        if no_galaxies {
            empty_cols.push(c);
        }
    }

    for r in 0..character_map.len() {
        if !character_map[r].contains(&'#') {
            empty_rows.push(r);
        }
    }

    println!("{:?}", empty_rows);
    println!("{:?}", empty_cols);

    let mut galaxies = Vec::new();
    let mut r_expanded = 0;
    for r in 0..character_map.len() {
        let r_plus = empty_rows.clone().into_iter().filter(|x| *x <= r).count();

        for c in 0..character_map[r].len() {
            let c_plus = empty_cols.clone().into_iter().filter(|x| *x <= c).count();
            if character_map[r][c] == '#' {
                galaxies.push((r+r_plus, c + c_plus));
            }
        }
    }

    let mut distances = Vec::new();
    for k in 0..galaxies.len() {
        for l in k..galaxies.len() {
            distances.push(get_distance(&galaxies[k], &galaxies[l]));
        }
    }
    let score: i64 =  distances.iter().sum();
    // println!("Distances {:?}",distances);
    println!("Puzzle 11 score = {}",score);

    let mut galaxies = Vec::new();
    let mut r_expanded = 0;
    for r in 0..character_map.len() {
        let r_plus = 999999*empty_rows.clone().into_iter().filter(|x| *x <= r).count();

        for c in 0..character_map[r].len() {
            let c_plus = 999999*empty_cols.clone().into_iter().filter(|x| *x <= c).count();
            if character_map[r][c] == '#' {
                galaxies.push((r+r_plus, c + c_plus));
            }
        }
    }

    let mut distances = Vec::new();
    for k in 0..galaxies.len() {
        for l in k..galaxies.len() {
            distances.push(get_distance(&galaxies[k], &galaxies[l]));
        }
    }
    let score: i64 =  distances.iter().sum();
    println!("Puzzle 11b score = {}",score);

}

fn get_distance(galaxy: &(usize, usize), other_galaxy: &(usize, usize)) -> i64 {
    ((galaxy.0 as i64) - (other_galaxy.0 as i64)).abs() + ((galaxy.1 as i64) - (other_galaxy.1 as i64)).abs()
}