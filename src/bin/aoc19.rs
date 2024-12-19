use std::collections::HashMap;
use std::fs;

fn find_pattern( target: &str, towels: &Vec<&str>, memo: &mut HashMap<String, bool> ) -> bool {
    if let Some(res) = memo.get(target) {
        return *res;
    }
    if target.len() == 0 {
        return true;
    }
    for twl in towels {
        if twl.len() > target.len() {
            continue;
        }
        if &target[0..twl.len()] == *twl {
            if find_pattern( &target[twl.len()..], towels, memo ) {
                memo.insert( target.to_string(), true);
                return true;
            };
        }
    }
    memo.insert( target.to_string(), false);
    false
}

fn count_patterns( target: &str, towels: &Vec<&str>, memo: &mut HashMap<String, u64> ) -> u64 {
    if let Some(res) = memo.get(target) {
        return *res;
    }
    if target.len() == 0 {
        return 1;
    }
    let mut solutions = 0;
    for twl in towels {
        if twl.len() > target.len() {
            continue;
        }
        if &target[0..twl.len()] == *twl {
            solutions += count_patterns(&target[twl.len()..], towels, memo );
        }
    }
    memo.insert( target.to_string(), solutions);
    solutions
}

fn main() {
//    let input = fs::read_to_string( "inputs/test").unwrap();
    let input = fs::read_to_string( "inputs/aoc19").unwrap();

    let mut lines = input.lines();
    let towels: Vec<&str>  = lines.next().unwrap().split( ", ").collect();
    lines.next();
    let patterns: Vec<&str> = lines.collect();

    let mut count = 0;
    for pattern in &patterns {
        let mut memo: HashMap<String,bool>  = HashMap::new();
        println!( "{pattern}");
        if find_pattern( pattern, &towels, &mut memo )  {
            count += 1;
        }
    }
    println!( "Part1 {count}");

    let mut count = 0;
    for pattern in &patterns {
        let mut memo: HashMap<String,u64>  = HashMap::new();
        println!( "{pattern}");
        count += count_patterns( pattern, &towels, &mut memo );
    }
    println!( "Part2 {count}");

}