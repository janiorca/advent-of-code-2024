use std::collections::HashMap;
use std::fs;

fn step( secret: u64 ) -> u64 {
    let step1 = (( secret << 6 ) ^ secret ) & 0x00ffffff;
    let step2 = (( step1 >> 5 ) ^ step1 ) & 0x00ffffff;
    let step3 = (( step2 << 11 ) ^ step2 ) & 0x00ffffff;
    step3
}

fn get_seq_values( mut secret: u64 ) -> HashMap<(i32,i32,i32,i32), i32> {
    let mut digits = vec! [ 0i32;2000];
    let mut deltas = vec! [ 0i32;2000];
    let mut seq_vals: HashMap<(i32,i32,i32,i32), i32> = HashMap::new();

    for i in 0..2000 {
        digits[ i as usize] = ( secret % 10 ) as i32;
        if i > 0 {
            deltas[ i as usize ] = digits[ i as usize ] - digits[ (i-1) as usize ];
        }
        secret = step( secret);
    }
    for i in 4..2000 {
        let key = (deltas[i-3], deltas[i-2], deltas[i-1], deltas[i]);
        if !seq_vals.contains_key( &key ) {
            seq_vals.insert( key, digits[i]);
        }
    }
    seq_vals
}

fn main() {
    let input = fs::read_to_string( "inputs/aoc22").unwrap();
    let mut total = 0;
    for line in input.lines() {
        let mut value = line.parse::<u64>().unwrap();
        for _it in 0..2000 {
            value = step(value);
        }
        total += value;
    }
    println!( "Part1: {total}");

    let mut all_seq_vals: Vec< HashMap<(i32,i32,i32,i32), i32>> = Vec::new();
    for line in input.lines() {
        let mut value = line.parse::<u64>().unwrap();
        let seq_values = get_seq_values(value);
        all_seq_vals.push(seq_values);
    }

    let mut best = 0;
    for a in -9..=9 {
        for b in -9..=9 {
            for c in -9..=9 {
                for d in -9..=9 {
                    let mut total = 0;
                    let key = (a,b,c,d);
                    for seq in &all_seq_vals {
                        if let Some( price ) = seq.get(&key) {

                            total += *price;
                        }
                    }
                    best = best.max( total );

                }
            }
        }
    }
    println!( "Part2: {best}");
}