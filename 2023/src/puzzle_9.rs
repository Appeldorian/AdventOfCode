use core::{panic, num};
use std::{fs, collections::VecDeque, cmp, fmt, time, hash::Hash, vec, f32::MIN_POSITIVE, arch::x86_64::_SIDD_POSITIVE_POLARITY};

pub fn solve() { 
    let input_string = fs::read_to_string("./data/puzzle_9.txt").expect("File read error");
    let mut line_iterator = input_string.lines();

    let mut game_sum = 0;
    for (index, line) in line_iterator.enumerate() {
        let mut pattern = Pattern::new(&line);

        println!("Pattern {}: ", index);
        pattern.print();
        pattern.extend(1);
        pattern.print();

        game_sum += pattern.get_last_element();
    }

    println!("Puzzle 9 score: {}", game_sum);

    
    let mut line_iterator = input_string.lines();
    let mut game_sum = 0;
    for (index, line) in line_iterator.enumerate() {
        let mut pattern = Pattern::new(&line);

        println!("Pattern {}: ", index);
        pattern.print();
        pattern.extend_left(1);
        pattern.print();

        game_sum += pattern.get_first_element();
    }

    println!("Puzzle 9 part 2 score: {}", game_sum);
}


struct Pattern {
    numbers: VecDeque<VecDeque<i64>>,
}

impl Pattern {
    fn new(pattern_string: &str) -> Self {
        let mut numbers = VecDeque::new();
        let mut top_vector = VecDeque::new();
        for part in pattern_string.split_ascii_whitespace() {
        //     println!("Parsing: {}", part);
            let part_number: i64 = part.parse().unwrap();
            top_vector.push_back(part_number);
        }
        let nr_top_elements = top_vector.len();
        numbers.push_back(top_vector);

        for _ in 0..nr_top_elements-1 {
            let vector_above = numbers.back().unwrap();

            if check_if_all_zeroes(&vector_above) {
                break;
            }

            assert!(vector_above.len() >= 2, "Vector above has less than 2 elements!");
            let mut diff_vector = VecDeque::with_capacity(vector_above.len() - 1);
            for k in 1..vector_above.len() {
                diff_vector.push_back(vector_above[k] - vector_above[k-1]);
            }
            numbers.push_back(diff_vector);
        }

        if !check_if_all_zeroes(&numbers.back().unwrap()) {
            panic!("Couldn't find pattern! Not enough diff levels for the number of elements.");
        }

        Self { numbers: numbers }
    }

    fn print(&self) {
        for row in self.numbers.iter() {
            for element in row {
                print!("{} ", element);
            }
            print!("\n ");
        }
    }

    fn get_first_element(&self) -> i64 {
        *self.numbers.front().unwrap().front().unwrap()
    }
    fn get_last_element(&self) -> i64 {
        *self.numbers.front().unwrap().back().unwrap()
    }

    fn extend(&mut self, nr_elements: usize) {
        for _ in 0..nr_elements {
            let nr_levels = self.numbers.len();
            self.numbers[nr_levels-1].push_back(0);
            for k in (0..nr_levels-1).rev() {
                let extension = self.numbers[k].back().unwrap() + self.numbers[k+1].back().unwrap();
                self.numbers[k].push_back(extension);
            }
        }
    }
    fn extend_left(&mut self, nr_elements: usize) {
        for _ in 0..nr_elements {
            let nr_levels = self.numbers.len();
            self.numbers[nr_levels-1].push_back(0);
            for k in (0..nr_levels-1).rev() {
                let extension = self.numbers[k].front().unwrap() - self.numbers[k+1].front().unwrap();
                self.numbers[k].push_front(extension);
            }
        }
    }

}

fn check_if_all_zeroes(vector_to_check: &VecDeque<i64>) -> bool {
    let mut all_zero = true;
    for element in vector_to_check {
        if *element != 0 {
            all_zero = false;
        }
    }

    all_zero
}