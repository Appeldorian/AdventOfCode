use std::{fs, collections::HashMap, cmp, fmt, time};

const format_spaces: usize = 3;
pub fn solve() { 
    // let times: Vec<u32> = vec![7,15,30];
    // let distances: Vec<u32> = vec![9,40,200];

    // let times: Vec<u32> = vec![59,68,82,74];
    // let distances: Vec<u32> = vec![543,1020,1664,1022];

    let times: Vec<u64> = vec![59688274];
    let distances: Vec<u64> = vec![543102016641022];

    let nr_games = times.len();

    let mut total_combinations = 1;
    for game in 0..nr_games {
        let mut counter = 0;
        for charge_time in 0..times[game] {
            let speed = charge_time;
            let go_time = times[game] - charge_time;
            let distance = speed*go_time;
            if distance > distances[game]{
                counter += 1;
            }
        }
        total_combinations *= counter;
        println!("Number of combinations: {} ", counter);
    }
    println!("Total number of combinations: {} ", total_combinations);
}