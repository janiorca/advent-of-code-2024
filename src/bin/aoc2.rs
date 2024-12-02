use std::fs;

fn is_safe( numbers: &Vec<i64>) -> bool {
    let mut iter = numbers.iter();
    let mut prev = iter.next().unwrap();
    let (mut safe_increasing, mut safe_decreasing) = (true, true);

    while let Some( curr ) = iter.next() {
        if curr >= prev || prev - curr > 3 {
            safe_decreasing = false
        }
        if curr <= prev || curr - prev > 3 {
            safe_increasing = false
        }
        prev = curr;
    }
    return safe_increasing || safe_decreasing
}
fn main() {
    let input = fs::read_to_string("inputs/aoc2").unwrap();
    let mut safe_count = 0;
    for line in input.lines() {
        let numbers: Vec<i64>  = line.split_whitespace().map(|x|x.parse::<i64>().unwrap()).collect();
        if is_safe( &numbers ) {
            safe_count += 1;
        }
    }
    println!( "Part1 safe:{}", safe_count);

    safe_count = 0;
    for line in input.lines() {
        let mut numbers: Vec<i64>  = line.split_whitespace().map(|x|x.parse::<i64>().unwrap()).collect();

        if is_safe( &numbers ) {
            safe_count += 1;
        } else {
            for skip in 0..numbers.len() {
                let mut variant = numbers.clone();
                variant.remove( skip );
                if is_safe( &variant) {
                    safe_count += 1;
                    break;
                }
            }
        }
    }
    println!( "Part2 safe:{}", safe_count);
}