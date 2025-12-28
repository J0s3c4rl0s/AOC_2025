use core::num;
use std::{char, cmp::max, fs::read_to_string, vec};

fn main() {

    pt2();
}

fn pt2() {
    let contents = read_to_string("input.txt").expect("Read file");
    let lines : Vec<&str> = contents
        .lines()
        .collect();

    // grab operations
    let operation_vec = lines[lines.len()-1].split_whitespace();

    let digit_matrix : Vec<Vec<char>> = lines[..lines.len()-1]
        .iter().map(|s| s.chars().collect())
        .collect();

    let mut sum = 0;
    let mut j = 0;
    
    // for each iteration
    for op in operation_vec {
        // iterate over columns until you reach whitespace 
        println!("Checking numbers from column {j}");
        let (numbers, new_j) = parse_numbers(j, &digit_matrix);

        j = new_j + 1;
        println!("Applying operation {} on numbers {:?}", op, numbers);
        sum += numbers
            .into_iter().reduce(|i, j| apply_operation(op, i, j))        
            .expect("apply op")
    }

    println!("Total is {sum}")

}

fn parse_numbers(j : usize, digit_matrix : &Vec<Vec<char>>) -> (Vec<isize>, usize) {
    let mut numbers = vec![];

    // add a number for each column
    for column in j..digit_matrix[0].len() {
        // parse column
        let num_str : String = parse_column(column, digit_matrix);

        // flush numbers and new column if we reach an empty column
        if num_str.trim().is_empty() {
            return (numbers, column);
        }

        println!("Parsing {}", num_str);
        numbers.push(num_str.trim().parse().expect("parse to num"));
    }

    (numbers, digit_matrix[0].len())
}

fn parse_column(j: usize, digit_matrix : &Vec<Vec<char>>) -> String {
    digit_matrix
        .iter().map(|row| row[j])
        .collect()
}


// fn parse_line(line : &str) -> Vec<Vec<String>> {
//     let mut num_string_groups = vec![];
//     let mut char_iter = line.chars().peekable();
    
//     let mut curr_num_group = vec![];
//     while let Some(c) = char_iter.next() {
//         if c.is_whitespace() {
//             // If next token is not whitespace then flush digits and start new group
//             if let Some(next_c) = char_iter.peek() {
//                 if !next_c.is_whitespace() {
//                     num_string_groups.push(curr_num_group.to_owned());
//                     curr_num_group = vec![];
//                     continue;
//                 }
//             }
//         }

//         curr_num_group.push(c.to_string().trim().to_owned());
//     }

//     println!("Num groups for this line {:?}", num_string_groups);
//     num_string_groups
// }

// fn pt2_shit() {
//     let contents = read_to_string("test.txt").expect("Read file");
//     let strings : Vec<Vec<&str>> = contents
//         .lines()
//         .map(|s| s.split_whitespace().collect())
//         .collect();

//     // grab operations
//     let operation_vec = &strings[strings.len()-1];

//     // Array of rows, each row is made up of arrays of str digits 
//     let rows : &Vec<Vec<Vec<String>>> = 
//         &strings[0..strings.len()-1]
//         .iter().map(|row| 
//             // Map each string number to a vector of digits
//             row.iter().map(|num_str| 
//                 num_str.chars().map(|character| character.to_string())
//                 .collect())
//             .collect())
//         .collect();

    
        
//     let first_row = rows[0].to_owned();

//     let remaining_rows = &rows[1..].to_owned();

//     let res : isize =
//         // Collect num strings from top to bottom, appending from the right
//         collect_num_strings(remaining_rows, first_row)
//         // Map number strings to actual numbers
//         .iter().map(parse_to_nums)
//         // Align operations with number groups
//         .zip(operation_vec)
//         // Apply each operation to its relevant group of number (add/multiple all in group)
//         .map(|(num_vec, op)| apply_operation_group(num_vec, op))
//         // Add up all intermediate results
//         .sum();

//     println!("Total is {}", res)
// }

// fn collect_num_strings(
//     rows: &[Vec<Vec<String>>],
//     first_row: Vec<Vec<String>>,
// ) -> Vec<Vec<String>> {
//     let res = rows.iter().fold(first_row, |acc, row| {
//         acc.into_iter()
//             .zip(row.iter())
//             .map(|(acc_nums, digits)| reverse_concat(acc_nums, digits.to_vec()))
//             .collect()
//     });

//     println!("Collected num strings are: {:?}", res);

//     return res
// }


// fn parse_to_nums(v: &Vec<String>) -> Vec<isize> {
//     v
//         .iter().map(|num_string| 
//             num_string.parse().expect("number"))
//         .collect()
// }

// fn apply_operation_group(num_vec: Vec<isize>, op: &str) -> isize {
//     num_vec
//         .into_iter()
//         .reduce(|acc, num| apply_operation(op, acc, num))
//         .expect("Success")
// }

// fn reverse_concat(acc : Vec<String>, digits: Vec<String>) -> Vec<String>{
//     let is_acc_longer = acc.len() > digits.len();
//     let new_len = max(acc.len(), digits.len());
//     let delta = if is_acc_longer {
//         acc.len() - digits.len()
//     } else {
//         digits.len() - acc.len()
//     };

//     let mut new_acc = vec![];

//     // println!("Concatenating acc {:?} with digits {:?}", acc, digits);
//     for i in 0..new_len {
//         let curr_entry = if is_acc_longer {
//             acc[i].to_owned()
//         } else {
//             digits[i].to_owned()
//         };

//         new_acc.push(if i >= delta {
//             curr_entry + if is_acc_longer {
//                 &digits[i - delta]
//             } else {
//                 &acc[i - delta]
//             }
//         } else {
//             curr_entry
//         }); 
//     }

//     println!("Result of concatenation: {:?}", new_acc);
//     new_acc
// }

fn pt1() {
    let contents = read_to_string("test.txt").expect("Read file");
    let strings : Vec<Vec<&str>> = contents
        .lines()
        .map(|s| s.split_whitespace().collect())
        .collect();
    
    // grab operations
    let operation_vec = &strings[strings.len()-1];
    
    let numbers : &Vec<Vec<isize>> = 
        &strings[0..strings.len()-1]
        .iter().map(|v| 
            v.iter().map(|s| 
                s.parse().expect("number"))
            .collect())
        .collect();
        
    let first_row = numbers[0].to_owned();

    let remaining_rows = &numbers[1..];

    let res : isize = remaining_rows
        // update each row by adding/multiplying/subtracting to accumulator 
        .iter().fold(first_row, |acc, row| 
            apply_operation_vec(operation_vec, &acc, row))
        // add results together
        .iter().sum();

    println!("Total is {}", res)
}

fn apply_operation_vec (operation_vec : &Vec<&str>, first_row : &Vec<isize>, second_row : &Vec<isize>) -> Vec<isize>{
    operation_vec.iter()
        .zip(first_row)
        .zip(second_row)
        .map(|((op, i), j)| apply_operation(op, i.to_owned(), j.to_owned()))
        .collect()
}

fn apply_operation(op : &str, i : isize, j : isize) -> isize {
    match op {
        "*" => i * j,
        "+" => i + j,
        _ => panic!()
    }
}