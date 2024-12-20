use std::collections::{HashMap};
use std::fs;
use std::hash::Hash;

fn find( map: &Vec<Vec<char>>, c: char) -> (i32,i32) {
    for y in 0..map.len(){
        for x in 0..map[0].len() {
            if map[y][x] == c {
                return (y as i32,x as i32)
            }
        }
    }
    panic!("Cant find {c}");
}
fn main() {
    let input = fs::read_to_string( "inputs/aoc20").unwrap();
    let mut map: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        map.push( Vec::from_iter( line.chars()));
    }

    let start = find( &map, 'S');
    let end = find( &map, 'E');

    let mut path: Vec<(i32,i32)> = Vec::new();
    let mut pos = start;
    loop{
        path.push( pos );
        if pos == end { break; }

        for dir in [ (-1i32,0i32), (1,0), (0,-1), (0,1)] {
            let new_pos = (pos.0 + dir.0, pos.1 + dir.1 );
            if map[ new_pos.0 as usize][new_pos.1 as usize]  == '#' {
                continue;
            }
            if path.len() >=2 {
                let back_pos = path[ path.len()-2];
                if back_pos.0 == new_pos.0 && back_pos.1 == new_pos.1 {
                    continue;
                }
            }
            pos = new_pos;
            break;
        }
    }

    let mut cheats_by: Vec<i32> = Vec::new();
    for sidx in 0..path.len()-2 {
        let pos = path[ sidx];
        for dir in [(-2i32, 0i32), (2, 0), (0, -2), (0, 2)] {
            let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
            if let Some(good_idx) = path.iter().position(|x| x.0 == new_pos.0 && x.1 == new_pos.1) {
                let cheat_q = good_idx as i32 - (sidx as i32 + 2);
                if cheat_q > 0 {
                    cheats_by.push(cheat_q);
                }
            }
        }
    }
    let total_cheats = cheats_by.iter().filter( |x| **x >= 100).count();
    println!( "Part1: {:?}", total_cheats);

    let mut path_set:  HashMap<(i32,i32), i32>  = HashMap::new();
    for (idx, pt) in path.iter().enumerate() {
        path_set.insert( *pt, idx as i32);
    }

    let mut cheats_by: Vec<i32> = Vec::new();
    for sidx in 0..path.len()-2 {
        let pos = path[sidx];
        for y_offset in -20i32..=20 {
            for x_offset in -20i32..=20 {
                let path_len = y_offset.abs() + x_offset.abs();
                if path_len > 20 {
                    continue;
                }
                let new_pos = (pos.0 + y_offset, pos.1 + x_offset);
                if let Some(good_idx) = path_set.get( &new_pos ) {
                    let cheat_q = *good_idx as i32 - (sidx as i32 + path_len);
                    if cheat_q > 0 {
                        cheats_by.push(cheat_q);
                    }
                }
            }
        }
    }
    let total_cheats = cheats_by.iter().filter( |x| **x >= 100 ).count();
    println!( "Part2: {:?}", total_cheats);


}