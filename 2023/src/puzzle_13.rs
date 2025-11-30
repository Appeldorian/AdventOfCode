use core::{panic, num};
use std::collections::HashMap;
use std::{fs};
use std::io::{BufWriter, Write};

use std::time::{Duration, SystemTime};


pub fn solve() {
    let input_string = fs::read_to_string("./data/puzzle_13.txt").expect("File read error");
    let lines = input_string.lines();

    let mut block_counter = 0;
    let mut total_counter = 0;
    let mut total_counter_part_2 = 0;
    let mut block: Vec<Vec<char>> = Vec::new();
    for (l, line) in lines.enumerate() {
        if line.is_empty() {
            let nr_rows = block.len();
            let nr_cols = block.first().unwrap().len();

            let mirror_row = find_mirror_line_horizontal(&block, 0);
            let mirror_col = find_mirror_line_vertical(&block, 0);
            let mirror_score = get_mirror_score(mirror_row, mirror_col);

            let mut mirror_score_smudged = 0;


            let mut mirror_row_smudged = Some(0);
            let mut mirror_col_smudged= Some(0);
            let mut smudged_block = block.clone();

            // Find new reflection line with smudge.
            'outer: for row in 0..nr_rows {
                for col in 0..nr_cols {
                    smudged_block = apply_smudge(&block, row, col);
                    mirror_row_smudged = find_mirror_line_horizontal(&smudged_block, mirror_row.unwrap_or(0));
                    mirror_col_smudged = find_mirror_line_vertical(&smudged_block, mirror_col.unwrap_or(0));

                    if mirror_row_smudged.is_some() && mirror_col_smudged.is_some() {
                        if mirror_row_smudged == mirror_row && mirror_col_smudged != mirror_col {
                            mirror_row_smudged = None;
                        }
                        else if mirror_col_smudged == mirror_col  && mirror_row_smudged != mirror_row {
                            mirror_col_smudged = None;
                        }
                    }

                    if mirror_row_smudged == mirror_row && mirror_col_smudged == mirror_col {
                        continue;
                    }

                    let mirror_score_smudged_temp = get_mirror_score(mirror_row_smudged, mirror_col_smudged);

                    if mirror_score_smudged_temp > 0 {
                        mirror_score_smudged = mirror_score_smudged_temp;
                        break 'outer;
                    }
                }
            }

            println!("Block {}", block_counter);
            if mirror_score_smudged == 0 {
                println!(" ---  No smudge mirror score found.");
                // display_block(&block);
                // 
                // mirror_score_smudged = mirror_score;
            }
            println!("Original:\n");
            display_block_with_mirror_lines(&block, mirror_row, mirror_col);
            println!();
            println!("Smudged:\n");
            display_block_with_mirror_lines(&smudged_block, mirror_row_smudged, mirror_col_smudged);
            println!();
            println!("Mirror score:         {}", mirror_score);
            println!("Smudged mirror score: {}", mirror_score_smudged);

            total_counter += mirror_score;
            total_counter_part_2  += mirror_score_smudged;
            block_counter += 1;
            block.clear();
            continue;
        }
        
        let parts: Vec<char> = line.chars().collect();
        block.push(parts);

    }
    println!("Puzzle 13 game sum = {}", total_counter);
    println!("Puzzle 13 part 2 game sum = {}", total_counter_part_2);
}

struct Block {
    block: Vec<Vec<char>>,
}

fn get_mirror_score(mirror_row: Option<usize>, mirror_col: Option<usize>) -> usize {
    let mut score = 0;
    if let Some(r) = mirror_row {
        score += 100*r;
    }
    if let Some(c) = mirror_col {
        score += c;
    }
    score
}

fn apply_smudge(block: &Vec<Vec<char>>, row_index: usize, col_index: usize) -> Vec<Vec<char>> {
    let mut smudged_block = block.clone();
    if smudged_block[row_index][col_index] == '.' {
        smudged_block[row_index][col_index] = '#';
    }
    else {
        smudged_block[row_index][col_index] = '.';
    }
    smudged_block
}

fn find_mirror_line_horizontal(block: &Vec<Vec<char>>, ignore_mirror: usize) -> Option<usize> {
    let mut mirror_candidate_row = None;
    let nr_rows = block.len();
    for r in 1..nr_rows {
        if r == ignore_mirror {
            continue;
        }
        if block[r] == block[r-1] {
            let mut is_mirror = true;
            for r2 in r+1..nr_rows {
                let mirrored_row_index = r.checked_sub(r2-r+1);
                if let Some(valid_mirrored_row) = mirrored_row_index {
                    if block[r2] != block[valid_mirrored_row] {
                        is_mirror = false;
                        break;
                    }
                }
                else {
                    break;
                }
            }

            if is_mirror {
                // println!("Mirror found at: {}", r);
                mirror_candidate_row = Some(r);
                break;
            }
        }
    }
    mirror_candidate_row
}



fn find_mirror_line_vertical(block: &Vec<Vec<char>>, ignore_mirror: usize) -> Option<usize> {
    let nr_rows = block.len();
    let nr_cols = block.first().expect("Block without columns!").len();

    let mut transposed_block = vec![vec!['0'; nr_rows]; nr_cols];

    for r in 0..nr_rows {
        for c in 0..nr_cols {
            transposed_block[c][r] = block[r][c];
        }
    }

    find_mirror_line_horizontal(&transposed_block, ignore_mirror)
}

fn invert_character(input: char) -> char {
    let mut output ;
    if input == '.' {
        output = '#';
    }
    else {
        output = '.';
    }
    output
}


fn display_block(block: &Vec<Vec<char>>) {
    for row in block {
        let row_string = String::from_iter(row.clone());
        println!("{}", row_string);
    }
}
fn display_block_with_mirror_lines(block: &Vec<Vec<char>>, mirror_row: Option<usize>, mirror_col: Option<usize>) {
    for (r, row) in block.iter().enumerate() {
        if let Some(mirror_row_valid) = mirror_row {
            if r == mirror_row_valid {
                for k in 0..row.len() {
                    print!("-")
                }
                println!();
            }
        }
        for (c, col) in row.iter().enumerate() {
            if let Some(mirror_col_valid) = mirror_col {
                if c == mirror_col_valid {
                    print!("|");
                }
            }
            print!("{}", col);
        }
        println!();
    }
    println!();
}