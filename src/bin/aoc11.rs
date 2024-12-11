use std::collections::{HashMap, HashSet};
use std::fs;

fn count( number: u64, blink: u64, memo: &mut HashMap<(u64,u64),u64> ) -> u64 {
    if blink == 0 {
        return 1;
    }
    if memo.contains_key(&(blink,number)) {
        return *memo.get(&(blink,number)).unwrap();
    }

    let ret;
    if number == 0 {
        ret = count( 1, blink-1, memo);
    } else {
        let number_str = number.to_string();
        if number_str.len() %2 == 0 {
            let (left_str, right_str) = number_str.split_at(number_str.len() / 2);
            let left = left_str.parse::<u64>().unwrap();
            let right = right_str.parse::<u64>().unwrap();
            ret = count(left, blink - 1, memo) + count(right, blink - 1, memo);
        } else {
            ret = return count(number*2024, blink-1, memo);
        }
    }
    memo.insert((blink,number), ret);
    return ret;
}

fn main() {
    let input = fs::read_to_string("inputs/aoc11").unwrap();
    let numbers: Vec<u64> = input.split_whitespace().map( |x| x.parse::<u64>().unwrap()).collect();
    let mut total = 0;

    let mut memo: HashMap<(u64,u64),u64> = HashMap::new();
    for number in &numbers {
        total += count( *number, 25, &mut memo );
    }
    println!( "Part 1: {total}");

    let mut memo: HashMap<(u64,u64),u64> = HashMap::new();
    for number in &numbers {
        total += count( *number, 75, &mut memo );
    }
    println!( "Part 2: {total}");
}