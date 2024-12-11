use std::collections::HashSet;
use std::fs;

fn visit( pos: (i32,i32), map: &Vec<Vec<i32>>) -> (HashSet<(i32,i32)>, u32) {
    let current_val = map[pos.0 as usize][pos.1 as usize];
    if current_val == 9 {
        return (HashSet::from( [pos]),1);
    }

    let mut result: (HashSet<(i32,i32)>,u32) = (HashSet::new(), 0);
    for dir in [ (0,1), (0,-1), (1,0), (-1,0)] {
        let next = (pos.0+dir.0, pos.1+dir.1);
        if next.0 >= map.len() as i32 || next.1 >= map.len() as i32 || next.0 < 0 || next.1 < 0 {
            continue;
        }
        if map[next.0 as usize][next.1 as usize] != map[pos.0 as usize][pos.1 as usize] + 1 {
            continue;
        }
        let nn: (HashSet<(i32,i32)>,u32)  = visit( next, map );
        result = ( HashSet::from_iter( result.0.union( &nn.0).cloned()), result.1 + nn.1);
    }
    return result;
}

fn  main() {
    let mut map: Vec<Vec<i32>> = Vec::new();
    let input = fs::read_to_string("inputs/aoc10").unwrap();
//    let input = fs::read_to_string("inputs/test").unwrap();
    for line in input.lines() {
        let row: Vec<i32> = line.chars().map( |c|c as i32 - '0' as i32).collect();
        map.push(row);
    }

    // Scan
    let mut total_dests = 0;
    let mut total_paths = 0;
    for sy in 0..map.len() {
        for sx in 0..map[0].len() {
            if map[sy][sx] == 0 {
                let (dests, paths) = visit( (sy as i32,sx as i32), &map );
                total_dests += dests.len() as u32;
                total_paths += paths;
            }
        }
    }
    println!( "Part 1: {} ", total_dests );
    println!( "Part 2: {} ", total_paths );
}