use std::{fs::read_to_string, result};



fn main() {
    let input : Vec<Vec<char>> = 
        read_to_string("input.txt")
        .expect("read file")
        .lines()
        .map(|s| s.chars().collect())
        .collect();

    pt2(input);
}

fn pt2(input : Vec<Vec<char>>) {
    // initialize first beam
    let beams : Vec<usize> = input[0]
        .iter().map(|c| match c {
            'S' => 1, 
            _ => 0
        }).collect();

    let result : usize = input[1..]
        .into_iter()
        .fold(beams, update_quantum_beams)
        .iter().sum();


    let result = result + 1;
    // let result = (result - 1) * 2;
    println!("Total number of timelines is {result}")
}

fn update_quantum_beams(old_beams : Vec<usize>, row : &Vec<char>) -> Vec<usize> {
    // I think this could be done in place but whatever
    let mut beams = old_beams.to_owned();

    // Check for each entry
    for (i, c) in row.iter().enumerate() {
        // If beam hits splitter
        if old_beams[i] > 0 && *c == '^' {
            // println!("hit!");
            // beam no longer at this position
            beams[i] = 0;
            // spawn beam to the left
            if i > 1 {
                // new beam
                beams[i-1] += old_beams[i];
            }
            // spawn beam to the right
            if i < beams.len()-1 {
                beams[i+1] += old_beams[i];
            }
        }
    }
    println!("New beams are {:?} \n for row {:?}", beams, row);
    beams
}

fn pt1(input : Vec<Vec<char>>) {
    // initialize first beam
    let beams : Vec<bool> = input[0]
        .iter().map(|c| match c {
            'S' => true, 
            _ => false
        }).collect();

    let (_final_beams, result) = input[1..]
        .into_iter()
        .fold((beams, 0), |(old_beams, counter), row| update_beams(old_beams, counter, row));


    println!("Total number of beams is {result}")
}

fn update_beams(old_beams : Vec<bool>, old_counter : usize, row : &Vec<char>) -> (Vec<bool>, usize) {
    // I think this could be done in place but whatever
    let mut beams = old_beams.to_owned();
    let mut counter = old_counter;

    // Check for each entry
    for (i, c) in row.iter().enumerate() {
        // If beam hits splitter
        if old_beams[i] && *c == '^' {
            // println!("hit!");
            counter += 1;
            // beam no longer at this position
            beams[i] = false;
            // spawn beam to the left
            if i > 1 {
                beams[i-1] = true;
            }
            // spawn beam to the right
            if i < beams.len()-1 {
                beams[i+1] = true;
            }
        }
    }
    // println!("New beams are {:?} \n for row {:?}", beams, row);
    (beams, counter)
}
