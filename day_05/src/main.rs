use std::{collections::{HashMap, HashSet}, fs::read_to_string, path::Path, pin, string, vec};


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Range(usize, usize);

fn main() {
    let (ranges, items) = read_input("input.txt");

    pt2(ranges);
}

fn pt2(mut ranges : Vec<Range>) {
    // Sort ranges by lower bound
    // ranges.sort_by(|t, o| t.0.cmp(&o.0));
    ranges.sort();

    // println!("Ranges : {:?}", ranges);

    let mut sum = 0;

    let mut prev_highest = 0;

    for (i, Range(low, high)) in ranges.iter().enumerate() {
        // dereference pointers already
        let (low, high) = (low.to_owned(), high.to_owned());
        
        // top bound is included so add one more to delta
        let delta = 1 + high - low;

        // Just add first range
        if i == 0  {
            sum += delta;
            prev_highest = high;
            continue;
        }


        // Check if contained in any previous highs
        if prev_highest >= high {
            continue;
        }

        // Update sum 
        sum +=delta - if low <= prev_highest {
            // top bound is included so add one more
            1 + prev_highest - low
        } else {
            0
        };

        // update highest
        prev_highest = high;
    }

    println!("Total is {}", sum)
}


fn pt2_stupid(ranges : Vec<Range>, items : Vec<usize>) {
    let mut pinged = HashSet::new();

    for Range(low, high) in ranges {
        for val in low..=high {
            pinged.insert(val);
        }
    }

    println!("Total fresh IDs: {}", pinged.len());
}

fn pt1(ranges : Vec<Range>, items : Vec<usize>) {
    let fresh_count : usize = items
        .iter().map(|item| check_item(&ranges, item.to_owned()))
        .sum();
    println!("Total: {}", fresh_count);
}

fn read_input(path : &str) -> (Vec<Range>, Vec<usize>) {
    let contents = read_to_string(path).expect("Read file");
    let strings : Vec<&str> = contents.lines().collect();

    let whitespace_index = strings.iter().position(|s| s.is_empty()).expect("Empty line");

    let str_ranges =  &strings[..whitespace_index];
    let str_items = &strings[whitespace_index + 1..];   // Read items

    (read_ranges(str_ranges.to_vec()), read_items(str_items.to_vec()))
}

fn read_ranges(str_ranges : Vec<&str>) -> Vec<Range> {
    str_ranges
        .iter()
        .map(|s| s.split('-'))
        .map(|mut v| 
            // Consume "split" and then do annoying checks to get string and conver to number
            Range(v.next().expect("Lower bound").parse().expect("Not a number"), v.next().expect("Upper bounds").parse().expect("Not a number")))
        .collect()
}

fn read_items(str_items : Vec<&str>) -> Vec<usize> {
    str_items
        .iter()
        .map(|s| s.parse().expect("Not a number"))
        .collect()
}


fn check_item(ranges : &Vec<Range>, item : usize) -> usize {
    // If any ranges match, return 1
    if ranges.iter().any(|Range(lower, higher)| item <= higher.to_owned() && item >= lower.to_owned()) {
        1
    } else {
        0
    }
}