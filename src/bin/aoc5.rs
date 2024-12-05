use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/aoc5").unwrap();
    let mut lines = input.lines();
    let mut rules: HashMap<i64, HashSet<i64>> = HashMap::new();
    loop {
        let line = lines.next().unwrap();
        if line == "" { break; }
        let mut rule_parts = line.split("|");
        let rule_key  = rule_parts.next().unwrap().parse::<i64>().unwrap();
        let rule_value = rule_parts.next().unwrap().parse::<i64>().unwrap();
        if !rules.contains_key( &rule_key ) {
            rules.insert( rule_key, HashSet::new() );
        }
        rules.get_mut( &rule_key ).unwrap().insert( rule_value);
    }

    let mut total = 0i64;
    while let Some( line ) = lines.next() {
        let update: Vec<i64> = line.split(",").map(|x|x.parse::<i64>().unwrap()).collect();

        let mut succeed = true;
        for number_pos in 1..update.len() {
            let on_left: HashSet<i64> = HashSet::from_iter( update[..number_pos].iter().cloned() );

            if let Some( number_rules )= rules.get(&update[number_pos]) {
                if on_left.intersection( &number_rules).count() > 0 {
                    succeed = false;
                    break;
                }
            }
        }
        if succeed {

            total += update.get( update.len()/2).unwrap();
        }
    }
    println!( "Patr1: {}", total);
}

