use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let mut map: Vec<Vec<char>> = vec![];

    let input = fs::read_to_string("inputs/aoc8").unwrap();
    for line in input.lines() {
        map.push(line.chars().collect());
    }
    let (height, width) = (map[0].len() as i64, map[1].len() as i64);

    let mut antennas: HashMap<char, Vec<(i64,i64)>> = HashMap::new();
    for y in 0..height {
        for x in 0..width {
            if map[y as usize][x as usize] != '.' {
                antennas.entry( map[y as usize][x as usize] ).or_insert(vec![]).push( (y,x) )
            }
        }
    }

    let mut answers = Vec::new();
    for  (start_dist, stop_dist) in [ (1,2), (0,width) ]{
        let mut resonant_points: HashSet<(i64, i64)> = HashSet::new();
        for antenna_set in antennas.values() {
            for left in 0..antenna_set.len() {
                for right in 0..antenna_set.len() {
                    if left == right { continue; }
                    let dist_vec = (antenna_set[right].0 - antenna_set[left].0, antenna_set[right].1 - antenna_set[left].1);
                    for (mut start, step) in [(antenna_set[left], (-dist_vec.0, -dist_vec.1)), (antenna_set[right], dist_vec)] {
                        for dist in start_dist..stop_dist {
                            let pos = (start.0 + step.0 * dist, start.1 + step.1 * dist);
                            if pos.0 >= 0 && pos.0 < height && pos.1 >= 0 && pos.1 < width {
                                resonant_points.insert(pos);
                            }
                        }
                    }
                }
            }
        }
        answers.push( resonant_points.len());
    }
    println!("Part1 {:?}", answers[0]);
    println!("Part2 {:?}", answers[1]);
}