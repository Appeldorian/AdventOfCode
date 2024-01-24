use core::panic;
use std::{fs, collections::HashMap, cmp, fmt, time, hash::Hash, vec, f32::MIN_POSITIVE, arch::x86_64::_SIDD_POSITIVE_POLARITY};

pub fn solve() { 
    let input_string = fs::read_to_string("./data/puzzle_8.txt").expect("File read error");
    let mut line_iterator = input_string.lines();

    let instructions = line_iterator.next().unwrap();
    line_iterator.next();

    let mut nodes = Vec::new();
    for line in line_iterator {
        nodes.push(Node::new(&line));
    }
    
    let mut position = find_position(&nodes, "AAA");
    let finish = find_position(&nodes, "ZZZ");
    let mut nr_steps = 0;
    println!("Start = {}", position.start);
    println!("Finish = {}", finish.start);

    'outer: while true {
        for instruction in instructions.chars() {
            let next_position = get_next_position(&nodes, &position, &instruction);
            if position.start == finish.start {
                println!("Finish reached!");
                println!("Puzzle 8 score: {}", nr_steps);
                break 'outer;
            }
            position = next_position;
            nr_steps += 1;
        }    
    }

    let mut positions = find_nodes_ending_in_letter(&nodes, &'A');
    let mut finish_positions = find_nodes_ending_in_letter(&nodes, &'Z');
    for position in positions.iter() {
        println!("{}", position.start);
    }
    for finish_position in finish_positions {
        println!("{}", finish_position.start);
    }

    let mut last_letters = String::new();
    let finish_positions = find_nodes_ending_in_letter(&nodes, &'Z');
    // let mut denominators = Vec::new();
    // println!("Searching: {} => {}", position.start, );
    let mut nr_steps = 0;    
    'outer: while true {
        for position in &mut positions {
            for instruction in instructions.chars() 
            {
                let next_position = get_next_position(&nodes, &position, &instruction);
                *position = *next_position;
                
            }
            nr_steps += 1;
            if check_if_all_finished(positions) {
                println!("Finished at: {}", position.start);
                break 'outer;
            }
        } 
    }
    // denominators.push(nr_steps);
    // let mut lcm_final = 1;

    // for d in denominators {
    //     lcm_final = lcm(lcm_final, d);
    //     println!("Nr steps: {}, lcm = {}", d, lcm_final);
    // }

        // lcms.push(lcm_final);

    
    println!("LCMS:");
    println!("Finish reached!");
    // println!("Puzzle 8 b score: {}", lcms.iter().min().unwrap());
}

#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub struct Node<'a> {
    start: &'a str,
    left: &'a str,
    right: &'a str,
}

impl<'a> Node<'a>  {
    pub fn new(line: &'a str) -> Self {
        Node { start: &line[0..3], left: &line[7..10], right: &line[12..15] }
    }
}

pub fn get_next_position<'a>(nodes: &'a Vec<Node>, current_position: &Node, instruction: &char) -> &'a Node<'a> {
    let mut target_position;
    if instruction == &'L' {
        // println!("{} : {} => {} ({}, {})", nr_steps, position.start, next_position(&nodes, &position.left).start, position.left, position.right);
        target_position = current_position.left;
    }
    else if instruction == &'R' {
        // println!("{} : {} => {} ({}, {})", nr_steps, position.start, next_position(&nodes, &position.right).start, position.left, position.right);
        target_position = current_position.right;
    }
    else {
        panic!("Instruction not L or R: {}", instruction);
    }

    find_position(nodes, target_position)
}

pub fn find_position<'a>(nodes: &'a Vec<Node>, target_position: &str) -> &'a Node<'a> {
    let mut position = nodes.first().unwrap();
    for node in nodes {
        if node.start == target_position {
            position = node;
        }
    }
    position
}

pub fn find_nodes_ending_in_letter<'a>(nodes: &'a Vec<Node>, letter: &char) -> Vec<Node<'a>> {
    let mut nodes_with_letter = Vec::new();
    for node in nodes {
        if node.start.chars().last().unwrap() == *letter {
            nodes_with_letter.push(node.clone());
        }
    }
    nodes_with_letter
}

pub fn check_if_all_finished(positions: &Vec<Node>) -> bool {
    for position in positions {
        if position.start.chars().last().unwrap() != 'Z' {
            return false;
        }
    }

    true
} 
pub fn lcm(mut n: u64, mut m: u64) -> u64{
    (n*m).checked_div(gcd(n, m)).unwrap()
}
pub fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
      if m < n {
        std::mem::swap(&mut m, &mut n);
      }
      m %= n;
    }
    n
  }
  