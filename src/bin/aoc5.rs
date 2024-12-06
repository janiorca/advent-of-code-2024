
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
        rules.entry( rule_key ).or_insert( HashSet::new()).insert( rule_value);
    }

    let mut total_1 = 0i64;
    let mut total_2 = 0i64;
    while let Some( line ) = lines.next() {
        let mut update: Vec<i64> = line.split(",").map(|x|x.parse::<i64>().unwrap()).collect();

        let mut moves = 0;
'outer:  loop{
            for number_pos in 1..update.len() {
                let on_left: HashSet<i64> = HashSet::from_iter(update[..number_pos].iter().cloned());

                if let Some(number_rules) = rules.get(&update[number_pos]) {
                    let inter: HashSet<i64> = HashSet::from_iter(on_left.intersection(number_rules).cloned().into_iter());
                    if inter.len() > 0 {
                        let first_offender = update.iter().position(|x| inter.contains(x)).unwrap();
                        update.swap( number_pos, first_offender);
                        moves += 1;
                        continue 'outer;
                    }
                }
            }
            break;
        }
        if moves == 0 {
            total_1 += update.get( update.len()/2).unwrap();
        } else {
            total_2 += update.get( update.len()/2).unwrap();
        }
    }
    println!( "Part1: {}", total_1);
    println!( "Part1: {}", total_2);
}