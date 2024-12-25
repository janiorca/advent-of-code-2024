use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Eq, Hash, PartialEq)]
struct Triplet{
    a: String,
    b: String,
    c: String,
}

impl Triplet {
    fn new( mut input: [String;3] ) -> Triplet {
        input.sort();
        Triplet{ a: input[0].clone(), b: input[1].clone(), c: input[2].clone() }
    }
}


fn order_r<'a, 'b>(mut left: &'a str, mut right: &'a str) -> (&'a str, &'a str ) {
    if right < left {
        let tmp = left;
        left = right;
        right = tmp;
    }
    ( left, right)
}

fn grow_sets( connected_sets: Vec<HashSet<String>>, connections: &HashMap<String,HashSet<String>>) -> Vec<HashSet<String>> {
    let mut new_sets: Vec<HashSet<String>> = Vec::new();

    let nodes: Vec<String> = connections.keys().cloned().collect();
'outer:
    for mut set in connected_sets {
        for pot_node in &nodes {
            if set.contains( pot_node ) {
                continue;
            }
            // Still here, new potential
            let conns = &connections[ pot_node ];
            let mut is_valid = true;
            for member in &set {
                if !conns.contains( member ) {
                    is_valid = false;
                    break;
                }
            }
            if is_valid {
                let mut c_set = set.clone();
                c_set.insert( pot_node.to_string() );
                if new_sets.iter().find( |x| **x == c_set).is_none() {
                    new_sets.push( c_set );
                    continue 'outer;
                // } else {
                //     println!( "Skipping existing")
                }
            }
        }
    }
    new_sets
}
fn main() {
    let input = fs::read_to_string( "inputs/aoc23").unwrap();

    let mut pairs: HashSet<(&str,&str)> = HashSet::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split( '-' ).collect();
        let mut left =  parts[0];
        let mut right =  parts[1];

        if left == right { continue; }
        (left,right) = order_r(left, right);
        pairs.insert( (&left,&right));
    }

    let mut triplets: HashSet<Triplet> = HashSet::new();
    for pair in &pairs {
        for other in &pairs {
            if pair == other {
                continue;
            }
            let mut points: HashMap<String, usize> = HashMap::new();
            for ends in &[ pair.0.clone(), pair.1.clone(), other.0.clone(), other.1.clone() ] {
                *points.entry(ends.to_string()).or_insert(0) += 1;
            }

            if points.len() == 3 {
                let needed: Vec<String> = points.iter().filter( |(key,count) | **count == 1 ).map( |x|x.0 ).cloned().collect();
                let p = order_r(&needed[0], &needed[1]);
                if pairs.contains( &p) {
                    let names: Vec<String> = points.keys().cloned().collect();
                    let triplet = Triplet::new( [ names[0].clone(), names[1].clone(),names[2].clone() ]);
                    triplets.insert( triplet );
                }
            }
        }
    }

    let mut num_t = 0;
    for triplet in &triplets {
        if triplet.a.starts_with('t') || triplet.b.starts_with( 't') || triplet.c.starts_with( 't') {
            num_t += 1;
        }
    }
    println!( "Part1: {num_t}");

    let mut connected_sets: Vec<HashSet<String>> = Vec::new();
    for pair in &pairs {
        let mut cs = HashSet::new();
        cs.insert( pair.0.to_string());
        cs.insert( pair.1.to_string());
        connected_sets.push(cs);
    }

    let mut connections: HashMap<String,HashSet<String>> = HashMap::new();
    for pair in &pairs {
        connections.entry( pair.0.to_string() ).or_insert( HashSet::new()).insert( pair.1.to_string());
        connections.entry( pair.1.to_string() ).or_insert( HashSet::new()).insert( pair.0.to_string());
    }

    loop{
        connected_sets = grow_sets( connected_sets, &connections);
        println!( "Len {}", connected_sets.len());
        if connected_sets.len() == 1 {
            break;
        }
    }
    let mut nodes: Vec<String> = Vec::from_iter( connected_sets[0].iter().cloned());
    nodes.sort();
    println!( "Part2: {}", nodes.join(","));
}