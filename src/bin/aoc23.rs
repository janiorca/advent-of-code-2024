use std::collections::{HashMap, HashSet};
use std::fs;
use std::ptr::swap;


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

fn order( mut left: String, mut right: String ) -> ( String, String ) {
    if right < left {
        let tmp = left;
        left = right;
        right = tmp;
    }
    ( left, right)
}

fn order_r<'a, 'b>(mut left: &'a str, mut right: &'a str) -> (&'a str, &'a str ) {
    if right < left {
        let tmp = left;
        left = right;
        right = tmp;
    }
    ( left, right)
}

fn main() {
//    let input = fs::read_to_string( "inputs/test").unwrap();
    let input = fs::read_to_string( "inputs/aoc23").unwrap();

//    let mut pairs: HashSet<(String,String)> = HashSet::new();
    let mut pairs: HashSet<(&str,&str)> = HashSet::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split( '-' ).collect();
        let mut left =  parts[0];
        let mut right =  parts[1];
        // let mut left =  parts[0].to_string();
        // let mut right =  parts[1].to_string();

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

    let mut pair_vecs: HashSet<Vec<String>> = HashSet::new();
    for pair in &pairs {
        let p_vec = Vec::from_iter( [pair.0.to_string(), pair.1.to_string()]);
        pair_vecs.insert( p_vec);
    }

    for pair in &pair_vecs {
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



    println!( "{}", pairs.len());
    println!( "{}", triplets.len());

    let mut num_t = 0;
    for triplet in &triplets {
        if triplet.a.starts_with('t') || triplet.b.starts_with( 't') || triplet.c.starts_with( 't') {
            num_t += 1;
        }
    }
    println!( "Part1: {num_t}");

}