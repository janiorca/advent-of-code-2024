use std::collections::{HashMap};
use std::fs;

fn grow_search( cost: i64, curr_loc: (i64,i64), map: &Vec<Vec<i64>>, visited: &mut HashMap<(i64,i64),i64>, searches: &mut Vec<((i64,i64), i64)>) {
    let dirs = [ (-1,0), (1,0), (0,-1), (0,1)];
    for dir in dirs {
        let new_loc = ( curr_loc.0 + dir.0, curr_loc.1 + dir.1 );
        if new_loc.0 < 0 || new_loc.1 < 0 || new_loc.0 >= map.len() as i64|| new_loc.1 >= map[0].len() as i64 {
            continue;
        }
        if map[ new_loc.0 as usize][ new_loc.1 as usize ] == 1 {
            continue;
        }
        if let Some( prev_cost ) = visited.get(&new_loc) {
            if *prev_cost <= cost+1 {
                continue;
            }
        }
        *visited.entry( new_loc).or_insert( cost+1 ) = cost + 1;
        searches.push( (new_loc, cost+1));
    }
}

fn find_solution( map: &Vec<Vec<i64>>, start: (i64,i64), stop: (i64,i64), find_best: bool) -> Option<u64> {
    let mut cheapest = 99999;
    let mut searches: Vec<((i64,i64), i64 )> = Vec::new();
    searches.push( (start, 0) );
    let mut visited: HashMap<(i64,i64),i64> = HashMap::new();

    while let Some(search) = searches.pop() {
        if search.0 == stop  {
            if !find_best {
                return Some(search.1 as u64)
            }
            if search.1 < cheapest {
                cheapest = search.1;
            }
        } else if search.1 < cheapest {
            grow_search( search.1, search.0,map, &mut visited, &mut searches);
        }
    }
    if cheapest == 99999 {
        return None;
    } else {
        return Some( cheapest as u64 )
    }
}

fn drop_stones( num_drops: usize, map: &mut Vec<Vec<i64>>, stones: &Vec<(i64,i64)> ) {
    for n in 0..num_drops {
        let drop = stones[n];
        map[ drop.0 as usize][ drop.1 as usize ] = 1;
    }
}

fn main() {
    let input = fs::read_to_string( "inputs/aoc18").unwrap();
    let (width, height) = (71,71);
    let num_drops = 1024usize;
    let mut map: Vec<Vec<i64>> = vec![vec![0;width];height];

    let mut drops: Vec<(i64,i64)> = Vec::new();
    for line in input.lines() {
        let mut parts = line.split(',');
        let x = parts.next().unwrap().parse::<i64>().unwrap();
        let y = parts.next().unwrap().parse::<i64>().unwrap();
        drops.push( (y,x))
    }
    drop_stones( num_drops, &mut map, &drops );

    let start = (0,0);
    let stop = ( height as i64 -1,width as i64-1);

    let cheapest  = find_solution( &map, start, stop, true ).unwrap();
    println!( "Part1  {}", cheapest);

    // Find the solution bt bisecting the search space
    let mut blo = 0;
    let mut bhigh = drops.len();
    loop{
        println!( "{blo} -- {bhigh}");
        if bhigh - blo == 1 {
            println!( "Part2: [ {},{} ] ", drops[ blo].1, drops[blo].0);
            break;
        } else {
            let new_idx = (blo + bhigh)/2;
            let mut new_map = map.clone();
            drop_stones( new_idx, &mut new_map, &drops );

            // Only care if thre IS a solution
            let probe = find_solution( &new_map, start, stop, false );
            if probe.is_none() { //} == false {
                bhigh = new_idx;
            } else {
                blo = new_idx;
            }
        }
    }
}