use std::{collections::BinaryHeap, fs};


fn main() {
    let input = fs::read_to_string("input/input.txt").unwrap();
    let total : usize = input
        .lines()
        .map(|bank| get_joltage_12digits(bank))
        .sum();
    println!("The total joltage is {total}");
}

fn get_joltage_12digits(bank: &str) -> usize {
    let mut last_idx = 0;
    let mut joltage = 0;
    for exp in (0..12).rev() {
        let (digit , idx) = get_max_digit(bank, last_idx, bank.len() - exp);
        last_idx = idx + 1; 
        joltage += digit * (10 as usize).pow(exp.try_into().unwrap());
    }

    println!("Current jultage for {bank} is {joltage}");
    joltage
}

fn _get_joltage_12digits(bank: &str) -> u64 {
    // let mut joltage = 0;

    let mut except = vec![false; bank.len()];
    // already sorted by first value?
    let mut digits = BinaryHeap::new();
    
    for _ in 1..=12 {
        let (val, idx) = get_max_digit_except(bank, &except);
        except[idx] = true;
        digits.push((idx, val));
    }

    let joltage = digits
        .into_sorted_vec()
        .iter()
        .fold("".to_owned(), |acc, (_, val)| acc + &val.to_string())
        .parse()
        .unwrap();

    println!("Current jultage for {bank} is {joltage}");

    joltage
}

// fn get_joltage_2digits(bank: &str) -> u32 {

//     let (dec_digit, except) = get_max_digit_except(bank, vec![false; bank.len()] );

//     let (unary_digit, _) = get_max_digit_except(bank, except);

//     let joltage = dec_digit * 10 + unary_digit;

//     println!("Current jultage for {bank} is {joltage}");
//     joltage
// }

fn get_joltage_2digits(bank: &str) -> usize {

    let (dec_digit, dec_idx) = get_max_digit(bank, 0, bank.len() - 1 );

    let (unary_digit, _) = get_max_digit(bank, (dec_idx + 1).try_into().unwrap(), bank.len());

    let joltage = dec_digit * 10 + unary_digit;

    println!("Current jultage for {bank} is {joltage}");
    joltage
}

fn get_max_digit_except(bank: &str, mut except: &Vec<bool>) -> (u64, usize){
    let (mut max_digit, mut max_idx) = (0, 0);

    for i in (0..bank.len()).rev() {
        // skip exception
        if except[i] {
            continue;
        }
        let bank_i  = bank.chars().nth(i).expect("Out of bounds").to_digit(10).expect("not a digit").try_into().unwrap();
        if bank_i > max_digit {
            max_digit = bank_i;
            max_idx = i;
        }
    }

    println!("Max digit {max_digit}, at idx {max_idx}");


    (max_digit, max_idx)
}

fn get_max_digit(bank: &str, from : usize, to : usize) -> (usize, usize){
    let (mut max_digit, mut max_idx) = (0, 0);

    for i in from..to {
        let bank_i = bank.chars().nth(i).expect("Out of bounds").to_digit(10).expect("not a digit").try_into().unwrap();
        if bank_i > max_digit {
            max_digit = bank_i;
            max_idx = i;
        }
    }

    println!("Max digit {max_digit}, at idx {max_idx}");

    (max_digit, max_idx)
}