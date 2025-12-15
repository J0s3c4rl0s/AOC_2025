use std::{cmp::{max, min}, fs::read_to_string, io::Lines, path::Path, vec};
fn main() {
    pt2();
}

fn pt2() {
    let mut input = read_input("input.txt");
    let mut count = 0;
    let mut edits = vec![];
    loop {
        for i in 0..input.len() {
            for j in 0..input[i].len() {
                if input[i][j] == 1 {
                    let works = check(&input, i, j);
                    
                    if works == 1 {
                        edits.push((i, j));
                    }
                    
                    count += works;
                }
            }        
        }

        // Stop when no more edits 
        if edits.is_empty() {
            break;
        }

        // Apply edits in place and empties edits vector
        edits.drain(..).for_each(|(i, j)| input[i.to_owned()][j.to_owned()] = 0);
    }
    println!("Final count is: {}", count);
    
}

fn pt1() {
    let input = read_input("input.txt");
    let mut count = 0;
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if input[i][j] == 1 {
                let works = check(&input, i, j);
                count += works;
            }
        }        
    }
    println!("Final count is: {}", count);

}

fn read_input(path: &str) -> Vec<Vec<isize>> {
    read_to_string(path)
        .expect("read file")
        .lines()
        // for each line
        .map(|s| 
            // map character to 0 or 1
            s.chars()
            .map(|c| if c == '.' {0} else {1})
            // collect inner line into a vector
            .collect())
        // collect outer into a vector 
        .collect()
}

fn check(rolls : &Vec<Vec<isize>>, i : usize, j : usize) -> usize {
    let mut sum = 0;
    for k in max(0 as isize, i as isize - 1)..=min((rolls.len() as isize)-1, i as isize + 1)   {
        for l in max(0 as isize, j as isize -1)..=min((rolls[i as usize].len() as isize)-1, j as isize + 1) {
            sum += rolls[k as usize][l as usize];
        }
    }

    (sum <= 4) as usize
}