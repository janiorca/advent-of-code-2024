use std::collections::BinaryHeap;
use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/aoc1").unwrap();
    let mut left: BinaryHeap<i64> = BinaryHeap::new();
    let mut right: BinaryHeap<i64> = BinaryHeap::new();
    for line in input.lines() {
        let mut number_strs = line.split_whitespace();
        let ll =number_strs.next().unwrap().parse::<i64>().unwrap();
        let rr =number_strs.next().unwrap().parse::<i64>().unwrap();
        left.push( ll );
        right.push( rr );
    }
    
    let mut total_distance = 0;
    for pair in left.clone().into_sorted_vec().iter().zip( right.clone().into_sorted_vec().iter()) {
        total_distance += (pair.0 - pair.1).abs();
    }
    println!("{}", total_distance);

    let mut total_similarity = 0;
    for l in left {
        let num_matches = right.iter().filter( |&&r|r==l).count();
        total_similarity += num_matches as i64 * l;
    }
    println!("{}", total_similarity);

}
