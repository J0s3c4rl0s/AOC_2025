use core::num;
use std::default;
use std::env;
use std::fs;
use std::i64;

fn main() {
    solution1();
    solution2();
}

fn solution2() {
    let input_string = fs::read_to_string("inputs/01.txt").expect("Could not read file");
    
    let mut counter = 0;

    let mut dial_value = 50; 
    for rotation in input_string.lines() {
        println!("Current rotation: {}", rotation);
        // Could byte slice bc ASCII but this is better practice
        let num_str : String = rotation.chars().skip(1).collect();
        let num : i64 = num_str.parse().expect("Not a number");

        let sign_str = rotation.chars().next().expect("Empty line");
        let sign : i64 =  match sign_str {
            'L' => -1, 
            'R' => 1,
            _ => panic!("First char neither L nor R: {sign_str}")
        };

        // Distance to 0 in our direction
        let delta =  if sign == 1 {100 - dial_value} else {dial_value};

        // Hits 0 at least once 
        if num >= delta {
            if delta != 0 {
                counter += 1;
            }
            println!("Crosses once");
            counter += (num - delta) / 100;
            println!("Crosses {}x", (num - delta) / 100);
        }

        dial_value = (dial_value + sign * num).rem_euclid(100);
        println!("new dial value: {}", dial_value);
        
        // counter = if dial_value == 0 {counter + 1} else {counter};
    }

    println!("{}", counter)

}

fn solution1 (){
    let input_string = fs::read_to_string("inputs/01.txt").expect("Could not read file");
    
    let mut counter = 0;

    let mut dial_value = 50; 
    for rotation in input_string.lines() {
        // println!("Current rotation: {}", rotation);
        // Could byte slice bc ASCII but this is better practice
        let num_str : String = rotation.chars().skip(1).collect();
        let num : i64 = num_str.parse().expect("Not a number");

        let sign_str = rotation.chars().next().expect("Empty line");
        let sign : i64 =  match sign_str {
            'L' => -1, 
            'R' => 1,
            _ => panic!("First char neither L nor R: {sign_str}")
        };

        dial_value = (dial_value + sign * num) % 100;
        // println!("new dial value: {}", dial_value);
        
        counter = if dial_value == 0 {counter + 1} else {counter};
    }

    println!("{}", counter)
}