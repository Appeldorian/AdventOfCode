use core::{panic, num};
use std::{fs, thread, time};

pub fn solve() { 
    let input_string = fs::read_to_string("./data/puzzle_10.txt").expect("File read error");

    let grid = get_grid(&input_string);
    let grid_borders = (grid.len(), grid.first().unwrap().len());
    let mut outline : Vec<Vec<char>> = vec![vec![' '; grid.first().unwrap().len()]; grid.len()];

    let outline_character = '.';
    let inner_tile_character = '$';

    let animal_position = find_animal(&grid);
    let animal_pipe = get_animal_pipe(&grid, &animal_position);
    // randomly start with first orientation
    let start_direction = *(get_orientations_from_pipe(&animal_pipe).first().unwrap());

    println!("Animal pipe = {} at position {}, {}, direction: {}", animal_pipe, animal_position.0, animal_position.1, direction_to_string(&start_direction));
    let mut position = animal_position.clone();
    let mut direction = start_direction.clone();
    outline[position.0][position.1] = match direction {
        Direction::Down => 'v',
        Direction::Up => '^',
        Direction::Right=> '>',
        Direction::Left => '<',
    };

    position = move_in_direction(&grid, &position, &direction);
    direction = get_next_direction(&grid, &position, &direction);
    outline[position.0][position.1] = match direction {
        Direction::Down => 'v',
        Direction::Up => '^',
        Direction::Right=> '>',
        Direction::Left => '<',
    };
    
    let outline_characters = vec!['v', '^', '>', '<'];

    let mut nr_steps: u64 = 1; // already moved one step forward before starting the loop (otherwise condition would be immediately fulfilled)
    while position != animal_position {
        position = move_in_direction(&grid, &position, &direction);
        nr_steps += 1;

        if get_pipe(&grid, &position) == 'S' {
            break;
        }
        outline[position.0][position.1] = match direction {
            Direction::Down => 'v',
            Direction::Up => '^',
            Direction::Right=> '>',
            Direction::Left => '<',
        };
        direction = get_next_direction(&grid, &position, &direction);
        // outline[position.0][position.1] = outline_character.clone();


        // for r in 0..outline.len() {
        //     for c in 0..outline.first().unwrap().len() {
        //         print!("{}", outline[r][c]);
        //     }
        //     println!();
        // }    
    }

    println!("Puzzle 10: Nr steps to go round: {}, so furthest point is at: {}", nr_steps, nr_steps.checked_div(2).unwrap());

    // let mut nr_dots_in_loop = 0;
    // Print outline for visualization.

    // To solve the second part of the puzzle, we need to know which pipes are part of the loop, to be able to count what's inside.
    // Outline now marks every pipe part of the outline with an x.

    let mut position = animal_position;
    let mut direction =  *(get_orientations_from_pipe(&animal_pipe).first().unwrap());
    
    position = move_in_direction(&grid, &position, &direction);
    direction = get_next_direction(&grid, &position, &direction);

    let mut nr_tiles_in_loop = 0;
    while position != animal_position {
        // move along loop
        position = move_in_direction(&grid, &position, &direction);

        // check w.r.t. the direction we came from
        // assume counterclockwise loop direction (found by trying both sides and seeing which one remains contained in the blob, rather than going to the edge)
        let dot_check_direction = match direction {
            Direction::Down => Direction::Right,
            Direction::Up => Direction::Left,
            Direction::Right=> Direction::Up,
            Direction::Left => Direction::Down,
        };
        
        let mut check_dot_position = move_in_direction(&grid, &position, &dot_check_direction);
        while !outline_characters.contains(&get_pipe(&outline, &check_dot_position)) {
            outline[check_dot_position.0][check_dot_position.1] = inner_tile_character.clone();
            check_dot_position = move_in_direction(&grid, &check_dot_position, &dot_check_direction);
        }

        direction = get_next_direction(&grid, &position, &direction);
        
        // check another time w.r.t. the direction we will go in to now. There are a few edge cases that would otherwise be missed if we didn't do this.
        // The edge cases are places where the loop goes around the enclosed tile without ever having it on the left side of the direction it came from,
        // so it requires a second check to see if it's on the left side of the direction it's going to. This happens when all the neighbouring pipes 
        // go in a direction away from the tile, such that it is never perpendicular to any pipe.
        let dot_check_direction = match direction {
            Direction::Down => Direction::Right,
            Direction::Up => Direction::Left,
            Direction::Right=> Direction::Up,
            Direction::Left => Direction::Down,
        };
        let mut check_dot_position = move_in_direction(&grid, &position, &dot_check_direction);
        // println!("Position: {}, {} = {} in {}", position.0, position.1, get_pipe(&grid, &position), direction_to_string(&direction));
        // println!("Checking dots for: {}, {} = {} in direction {}", check_dot_position.0, check_dot_position.1,get_pipe(&grid, &check_dot_position), direction_to_string(&dot_check_direction));
        while !outline_characters.contains(&get_pipe(&outline, &check_dot_position)) {
            // println!("Tile found: {}, {} = {}", check_dot_position.0, check_dot_position.1, get_pipe(&grid, &check_dot_position));
            outline[check_dot_position.0][check_dot_position.1] = inner_tile_character.clone();
            check_dot_position = move_in_direction(&grid, &check_dot_position, &dot_check_direction);
        }

    }

    // Fill gaps of enclosed tiles that were missed by the algorithm.
    for r in 0..outline.len() {
        for c in 0..outline.first().unwrap().len() {
            if outline[r][c] == ' ' {
                let mut start_row= 0;
                let mut end_row= grid.len();
                let mut start_col= 0;
                let mut end_col= grid[r].len();
                if r > 1 {
                    start_row = r-1;
                }
                if r + 2 <= grid.len() {
                    end_row = r + 2;
                }
                if c > 1 {
                    start_col = c-1;
                }
                if c + 2 <= grid[r].len() {
                    end_col = c + 2;
                }
                for row in start_row..end_row {
                    for col in start_col..end_col {
                        if outline[row][col] == '$' {
                            outline[r][c] = '$';
                        }
                    }
                }
            }
        }
        println!();
    }

    for r in 0..outline.len() {
        for c in 0..outline.first().unwrap().len() {
            print!("{}", outline[r][c]);
            if outline[r][c] == '$' {
                nr_tiles_in_loop += 1;
            }
        }
        println!();
    }
    

    // Om dit op te lossen: alles wat links ligt, relatief tot de voortbewegingsrichting, ligt buiten de loop.
    // Alles wat rechts ligt, binnen de loop. Maak hier gebruik van door alle tiles met . te scannen of ze links of rechts van de voortbeweginsrichting liggen.
    println!("Puzzle 10 b : {}", nr_tiles_in_loop);
}
#[derive(PartialEq, Eq, Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn direction_to_string(direction: &Direction) -> &str {
    match direction {
        Direction::Up => "up",
        Direction::Left => "left",
        Direction::Down => "down",
        Direction::Right => "right",
    }
}

fn get_inverse_direction(direction: &Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Down,
        Direction::Left => Direction::Right,
        Direction::Down => Direction::Up,
        Direction::Right => Direction::Left,
    }
}

fn move_in_direction(grid: &Vec<Vec<char>>, pos: &(usize, usize), direction: &Direction) -> (usize, usize) {
    let grid_borders = (grid.len(), grid.first().unwrap().len());
    let destination = match direction {
        Direction::Up => (sub_clip(pos.0, 1), pos.1),
        Direction::Right => (pos.0, add_clip(pos.1, 1, grid_borders.1)),
        Direction::Down => (add_clip(pos.0, 1, grid_borders.0), pos.1),
        Direction::Left => (pos.0, sub_clip(pos.1, 1)),
    };


    destination
}

fn get_next_direction(grid: &Vec<Vec<char>>, pos: &(usize, usize), came_from_direction: &Direction) -> Direction {
    let pipe = get_pipe(&grid, &pos);
    let mut pipe_orientations = get_orientations_from_pipe(&pipe);
    // println!("Pipe {}, len: {}", pipe, pipe_orientations.len());
    assert!(pipe_orientations.len() <= 2 && pipe_orientations.len() > 0);
    
    let mut came_from = 0; 
    for k in 0..pipe_orientations.len() {
        if *(pipe_orientations.get(k).unwrap()) == get_inverse_direction(came_from_direction) {
            came_from = k;
            break;
        }
    }
    pipe_orientations.remove(came_from);

    *(pipe_orientations.first().unwrap())
}

fn get_pipe(grid: &Vec<Vec<char>>, position: &(usize, usize)) -> char {
    let mut pipe = grid.get(position.0).unwrap().get(position.1).unwrap().clone();
    if pipe == 'S' {
        pipe = get_animal_pipe(&grid, &position);
    };
    pipe
}

fn get_grid(input_string: &str) -> Vec<Vec<char>> {
    let mut line_iterator = input_string.lines();
    let mut grid = Vec::new();
    for line in line_iterator {
        let mut row = Vec::new();
        
        for c in line.chars() {
            row.push(c);
        }

        grid.push(row);
    }

    grid
}


fn find_animal(grid: &Vec<Vec<char>>) -> (usize, usize) {
    let mut animal_position = (0,0);
    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            let value = grid.get(r).unwrap().get(c).unwrap();
            if value == &'S' {
                if animal_position != (0,0) {
                    print!("Multiple animals found: {}, {} and {}, {}", animal_position.0, animal_position.1, r, c);
                }
                println!("Animal found at {}, {}",r,c);
                animal_position = (r,c);
            }
        } 
    }

    assert!(animal_position != (0,0), "No animal found!");

    animal_position
}

fn get_animal_pipe(grid: &Vec<Vec<char>>, animal_position: &(usize, usize)) -> char {
    let mut animal_orientations = Vec::new();
    
    let up = get_orientations_from_pipe(grid.get(add_clip(animal_position.0, 1, grid.len())).unwrap().get(animal_position.1).unwrap());
    let right = get_orientations_from_pipe(grid.get(animal_position.0).unwrap().get(add_clip(animal_position.1, 1, grid.get(animal_position.0).unwrap().len())).unwrap());
    let down = get_orientations_from_pipe(grid.get(sub_clip(animal_position.0, 1)).unwrap().get(animal_position.1).unwrap());
    let left = get_orientations_from_pipe(grid.get(animal_position.0).unwrap().get(sub_clip(animal_position.1, 1)).unwrap());
    
    if up.contains(&Direction::Down) {
        animal_orientations.push(Direction::Up);
    }
    if right.contains(&Direction::Left) {
        animal_orientations.push(Direction::Right);
    }
    if down.contains(&Direction::Up) {
        animal_orientations.push(Direction::Down);
    }
    if left.contains(&Direction::Right) {
        animal_orientations.push(Direction::Left);
    }

    let pipe_types = vec!['|', '-', 'L', 'J', '7', 'F'];
    let mut animal_pipe = 'S';
    for pipe_type in pipe_types {
        let pipe_orientations = get_orientations_from_pipe(&pipe_type);
        let mut correct_pipe = true;
        for orientation in pipe_orientations {
            if !animal_orientations.contains(&orientation) {
                correct_pipe = false;
                break;
            }
        }
        if correct_pipe {
            animal_pipe = pipe_type;
        }

    }
    if animal_pipe == 'S' {
        panic!("Animal pipe not found!");
    }

    animal_pipe
}

fn get_orientations_from_pipe(pipe: &char) -> Vec<Direction> {
    let orientations = match pipe {
        '|' => vec![Direction::Up, Direction::Down],
        '-' => vec![Direction::Left, Direction::Right],
        'L' => vec![Direction::Up, Direction::Right],
        'J' => vec![Direction::Up, Direction::Left],
        '7' => vec![Direction::Left, Direction::Down],
        'F' => vec![Direction::Right, Direction::Down],
        _ => vec![],  
    };

    orientations
}

fn add_clip(pos: usize, add: usize, border: usize) -> usize {
    let mut value= pos.checked_add(add).unwrap();
    if value >= border {
        value = border;
    }

    value
}
fn sub_clip(pos: usize, sub: usize) -> usize {
    let mut value= pos.checked_sub(sub).unwrap();
    value
}