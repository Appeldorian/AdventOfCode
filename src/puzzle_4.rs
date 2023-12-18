use std::{fs, collections::HashMap, cmp, fmt, time};

#[test]
fn test(){
    assert!( 1 + 1 == 2);
}

const format_spaces: usize = 3;
pub fn solve() { 
    let input_string = fs::read_to_string("./data/puzzle_4.txt").expect("File read error");
    let mut line_iterator = input_string.lines();
    let mut lines_to_skip: u32 = 0;
    let mut transformation_index = 0;

    let state_names = [String::from("seed"), String::from("soil"), String::from("fertilizer"), String::from("water"),String::from("light"), String::from("temperature"), String::from("humidity"), String::from("location")];

    let mut transformations = HashMap::<String, Vec::<Transformation>>::new();

    // Collect seeds
    let line_string = line_iterator.next().expect("Error reading first line.");
    let seed_string = line_string.split_at(String::from("seeds: ").chars().count()).1;

    let seeds_part1 = get_seeds_part1(seed_string);
    let seeds_part2 = get_seeds_part2(seed_string);
    

    // Parse transitions
    let mut line_string = line_iterator.next().expect("Error reading next line.");
    for transition_index in 0..(state_names.len()-1)
    {
        if transformations.len() == state_names.len() - 1 {
            break;
        }
        let mut current_transformation = Vec::<Transformation>::new();
        let current_state = state_names.get(transformations.len()).expect("Bad index for state_names").clone();
        let next_state = state_names.get(transformations.len() + 1).expect("Bad index for state_names").as_str().clone();
        let transition_name = current_state.clone() + "-to-" + next_state.clone() + " map:";
        
        while !line_string.contains(&transition_name) {
            line_string = line_iterator.next().expect("Error reading next line.");
        }

        while !line_string.trim().is_empty()
        {
            let line_string = match(line_iterator.next())
            {
                Some(x) => x,
                None => {
                    break;
                }
            };

            println!("Reading number line: {:?}", &line_string);
            let numbers_string = line_string.split_whitespace();
            let numbers_vector: Vec<u64> = numbers_string.into_iter()
                                                            .map(|x| x.parse::<u64>().expect("Bad integer in transformation"))
                                                            .collect();

            
            if numbers_vector.len() == 3 {
                let dst = *numbers_vector.get(0).expect("Can't read destination.");
                let src = *numbers_vector.get(1).expect("Can't read source.");
                let tlen = *numbers_vector.get(2).expect("Can't read length.");
                
                current_transformation.push(Transformation::new(src, dst, tlen));
            }
            else {
                break;
            }
        }

        transformations.insert(current_state, current_transformation);
        println!("Transformations found: {}", transformations.len());
    }

    let locations_part1 = apply_transformations_to_seeds(seeds_part1, &transformations, &state_names);
    let locations_part2 = apply_transformations_to_seeds(seeds_part2, &transformations, &state_names);
    
    println!("Part 1:");
    let min_location = locations_part1.iter().min().expect("Can't find the minimum.");
    println!("Min location = {}", min_location);
    println!("Part 2:");
    let min_location2 = locations_part2.iter().min().expect("Can't find the minimum.");
    println!("Min location = {}", min_location2);

    // for (index, seed) in seeds.iter().enumerate() {
    //     println!("{} -> {}", &seed, &locations[index]);
    // }
}

fn get_seeds_part1(seed_string: &str) -> Vec<u64> {
    let seeds = seed_string.split_whitespace().map(|x| x.parse::<u64>().expect("Bad integer in seed string")).collect();
    println!("{:?}", seeds);
    seeds
}

fn get_seeds_part2(seed_string: &str) -> Vec<u64> {
    let seed_ranges: Vec<u64> = seed_string.split_whitespace().map(|x| x.parse::<u64>().expect("Bad integer in seed string")).collect();
    let mut seeds = Vec::new();
    for k in (0..seed_ranges.len()).step_by(2) {
        let seed_start = seed_ranges[k];
        let seed_length = seed_ranges[k+1];
        let seed_range =seed_start..seed_start+seed_length;
        seeds.extend(seed_range);
    }
    println!("{:?}", seeds);
    seeds
}

fn apply_transformations_to_seeds(seeds: Vec<u64>, transformations: &HashMap<String,Vec<Transformation>>, state_names: &[String; 8]) -> Vec<u64> {
    
    println!("Seeds: {:?}", &seeds);
    // println!("Transformations: {:?}", &transformations);
    
    let mut current_layer = seeds.clone();
    let mut locations = seeds.clone();
    for (state_index, state) in state_names[..state_names.len()-1].iter().enumerate() {
        println!("Transforming {} to {}", state, state_names[state_index+1]);
        let transformation_layer = transformations.get(state).unwrap();
        for (index, seed) in seeds.iter().enumerate() {
            for transformation in transformation_layer.iter() {
                // println!("Transforming {}", current_layer[index]);
                println!("Transformation: {}", &transformation);
                // println!("{} <= ", format!("{: >width$}",  transformation.src, width=format_spaces));
                // println!("{} <", format!("{: >width$}",  &locations[index], width=format_spaces));
                // println!("{}", format!("{: >width$}",  transformation.src + transformation.tlen, width=format_spaces));
                // println!("Before: {}", locations[index]);
                locations[index] = transformation.apply(current_layer[index].clone());
                if locations[index] != current_layer[index]{
                    break;
                }
                // println!("After:  {}", locations[index]);
                // println!(" -> ");
                // println!("{}",format!("{: >width$}",  &locations[index], width=format_spaces));
                // println!("------------------------------");
                // let ten_millis = time::Duration::from_millis(100);
                // let now = time::Instant::now();

                // thread::sleep(ten_millis);
            }
        }
        println!("{:?}", current_layer);
        println!("{:?}", locations);
        current_layer = locations.clone();
    }
    locations
}

#[derive(Clone, Debug)]
struct Transformation {
    src: u64,
    dst: u64,
    tlen: u64,
}

impl Transformation {
    fn new(src: u64, dst: u64, tlen: u64) -> Self {
        Transformation {src, dst, tlen}
    }
    fn apply(&self, value: u64) -> u64 {
        if value >= self.src && value < self.src + self.tlen {
            println!("{} -> {}", value, self.dst + (value - self.src));
            self.dst + (value - self.src)
        } 
        else {
            value
        }
    }

    
}


impl fmt::Display for Transformation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {} -> {} - {})", format!("{: >width$}", self.src, width=format_spaces), format!("{: >width$}", self.src+self.tlen, width=format_spaces), format!("{: >width$}", self.dst, width=format_spaces), format!("{: >width$}", self.dst+self.tlen, width=format_spaces))
    }
}